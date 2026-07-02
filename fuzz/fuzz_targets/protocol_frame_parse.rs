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

//! Fuzz target: `RiFrame::from_bytes`
//!
//! Strategy: feed the protocol frame parser with arbitrary bytes and look
//! for panics, integer overflows, OOM, or unchecked allocation requests.
//! This is the most exposed attack surface: a remote attacker controls the
//! raw byte stream arriving at the network layer.

use libfuzzer_sys::fuzz_target;
use ri::protocol::frames::RiFrame;

fuzz_target!(|data: &[u8]| {
    // Touch the parser. We deliberately do not inspect the result; the goal
    // is to surface any panic, abort, or allocator failure inside the call.
    let _ = RiFrame::from_bytes(data);
});
