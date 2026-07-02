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

//! Fuzz target: `RiFrameHeader::from_bytes`
//!
//! Strategy: the 32-byte header is parsed on every inbound packet before
//! any other check. Even a small parser bug here is reachable by an
//! attacker. This target focuses the search on the header parser in
//! isolation, which produces smaller corpora and faster repros.

use libfuzzer_sys::fuzz_target;
use ri::protocol::frames::RiFrameHeader;

fuzz_target!(|data: &[u8]| {
    let _ = RiFrameHeader::from_bytes(data);
});
