//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#![no_main]

//! Fuzz target: arbitrary regex pattern + matching input
//!
//! Strategy: this is the *generalized* regex DoS probe. We let the
//! fuzzer invent both a regex pattern and a string to match it against.
//! The harness runs the match on a worker thread with a 1 second join
//! timeout; if the match does not return in time, the worker is
//! abandoned and the input is recorded as a ReDoS finding via a panic.
//!
//! The timeout guard prevents a single runaway match from monopolising
//! the fuzzer's CPU budget, which is the failure mode of an ungoverned
//! ReDoS detector.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    let split = ((data[0] as usize) % (data.len() - 1)) + 1;
    let (pattern_bytes, input_bytes) = data.split_at(split);
    let pattern_str = String::from_utf8_lossy(pattern_bytes);
    let input_str = String::from_utf8_lossy(input_bytes);

    if pattern_str.len() > 512 || input_str.len() > 8 * 1024 {
        return;
    }
    if let Ok(re) = regex::Regex::new(&pattern_str) {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let _ = re.is_match(&input_str);
            let _ = tx.send(());
        });
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {}
            Err(_) => {
                // Abandon the worker; the join handle will be dropped.
                // Panic so libfuzzer records this input as a finding.
                panic!("regex DoS: pattern={:?} input_len={}", pattern_str, input_str.len());
            }
        }
    }
});
