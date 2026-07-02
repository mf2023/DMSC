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

//! Fuzz target: `serde_json::from_slice`
//!
//! Strategy: many internal Ri APIs (config reload, cache values, websocket
//! frames) accept a JSON payload that ultimately reaches `serde_json`.
//! The most common JSON deserialization bugs are:
//!   * deeply nested arrays/objects triggering stack overflow,
//!   * strings whose length field is corrupted (denial of service via a
//!     large allocation),
//!   * numbers with absurd exponents.
//! A panic in any of these paths is a finding.

use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    // Cap input to 64 KiB so a single allocation cannot OOM the harness.
    if data.len() > 64 * 1024 {
        return;
    }
    let _ = serde_json::from_slice::<Value>(data);
});
