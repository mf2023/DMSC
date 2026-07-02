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

//! Fuzz target: `RiBaggage::from_header` (W3C Baggage)
//!
//! Strategy: the baggage parser uses `item.find('=')` and slices on the
//! returned byte offset. Although the standard guarantees `find` returns
//! a valid UTF-8 boundary, defense in depth demands we still verify that
//! no input — including adversarial multi-byte sequences — can trigger a
//! panic in the integration. We also probe the round-trip to make sure
//! `to_header` is total.

use libfuzzer_sys::fuzz_target;
use ri::observability::propagation::RiBaggage;

fuzz_target!(|data: &[u8]| {
    if data.len() > 64 * 1024 {
        return;
    }
    let header = String::from_utf8_lossy(data);
    let baggage = RiBaggage::from_header(&header);
    // Round-trip is the only behavioral oracle we have for a permissive
    // parser. The fuzzer will rapidly enumerate pathologically long
    // keys/values, deeply nested keys, and so on.
    let _ = baggage.to_header();
    // And the iteration order must not panic.
    let _ = baggage.len();
    let _ = baggage.is_empty();
});
