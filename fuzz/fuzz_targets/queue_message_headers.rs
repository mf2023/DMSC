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

//! Fuzz target: `RiQueueMessage::with_headers`
//!
//! Strategy: the implementation truncates overlong header keys and values
//! to fixed byte caps with `&s[..N]`. If the cap does not land on a UTF-8
//! character boundary, the resulting slice panics. This target builds a
//! message from arbitrary bytes interpreted as UTF-8 strings (lossy) and
//! hammers the constructor. Other invariants the harness watches for:
//!   * control character handling must not index out of bounds,
//!   * the per-key truncation must never reach into multibyte sequences,
//!   * repeated inserts must respect the MAX_HEADERS ceiling without
//!     re-allocation blow-up.

use std::collections::HashMap;
use libfuzzer_sys::fuzz_target;
use ri::queue::core::RiQueueMessage;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    // Build a small header map by treating the fuzzer bytes as alternating
    // length-prefixed key/value pairs. Empty keys/values are skipped.
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    let mut count = 0;
    while i < data.len() && count < 256 {
        let key_len = (data[i] as usize) % 1024;
        i += 1;
        if i + key_len > data.len() {
            break;
        }
        let key = String::from_utf8_lossy(&data[i..i + key_len]).into_owned();
        i += key_len;
        if i >= data.len() {
            break;
        }
        let val_len = (data[i] as usize) % 4096;
        i += 1;
        if i + val_len > data.len() {
            break;
        }
        let value = String::from_utf8_lossy(&data[i..i + val_len]).into_owned();
        i += val_len;
        if !key.is_empty() {
            headers.insert(key, value);
        }
        count += 1;
    }

    let _ = RiQueueMessage::new(vec![0u8; 16]).with_headers(headers);
});
