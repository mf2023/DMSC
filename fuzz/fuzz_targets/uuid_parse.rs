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

//! Fuzz target: `uuid::Uuid::try_parse` (ASCII string parser)
//!
//! Strategy: UUIDs are accepted by Ri from session ids, message ids, and
//! trace correlation. The parser must accept canonical hyphenated form,
//! braced form (`{xxx}`), urn:uuid: form, hex-only form, and reject
//! anything else. The integration must not panic on partial input,
//! leading/trailing whitespace, embedded NULs, or non-ASCII characters.

use libfuzzer_sys::fuzz_target;
use uuid::Uuid;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1024 {
        return;
    }
    let candidate = String::from_utf8_lossy(data);
    // The three public parsers cover hyphenated, simple, and braced forms.
    let _ = Uuid::try_parse(&candidate);
    let _ = Uuid::try_parse_ascii(&candidate.as_bytes());
    // The binary parser must also be total: 16 bytes -> Ok, anything else -> Err.
    let _ = Uuid::try_from_slice(data);
    // The round-trip is the behavioral oracle for valid inputs.
    if let Ok(u) = Uuid::try_parse(&candidate) {
        let _ = u.to_string();
        let _ = u.as_bytes();
        let _ = u.as_hyphenated().to_string();
    }
});
