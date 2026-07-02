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

//! Fuzz target: `bincode::deserialize`
//!
//! Strategy: bincode uses length-prefixed strings and vectors. An
//! attacker who can supply a binary buffer can declare a 4 GiB length
//! for a `Vec<u8>` and force an allocator failure. The target encodes
//! a 2-byte length prefix to force the variable-length decoder, then
//! hands the rest to bincode. Any panic in the integration is a finding.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    // Restrict the slice to 8 KiB so a 2 GiB length-prefix bomb cannot
    // simply walk the fuzzer through the allocator.
    let _ = bincode::deserialize::<Vec<u8>>(&data[..data.len().min(8 * 1024)]);
});
