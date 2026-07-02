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

//! Fuzz target: `RiJwt::validate_token`
//!
//! Strategy: forge arbitrary JWT-shaped strings (header.payload.signature)
//! and feed them to the validator. The goal is to surface parser bugs in
//! the underlying `jsonwebtoken` crate integration and any panics in claim
//! deserialization that could be triggered by an attacker-controlled
//! Authorization header.

use libfuzzer_sys::fuzz_target;
use ri::auth::jwt::RiJwt;

fuzz_target!(|data: &[u8]| {
    // A valid token must be UTF-8; lossy conversion keeps the fuzzer simple
    // and still exercises the parser with every possible byte sequence.
    let candidate = String::from_utf8_lossy(data);
    // The validator must reject malformed input without panicking, leaking
    // information, or returning inconsistent error states.
    let jwt = RiJwt::new("fuzz-secret-do-not-use-in-production".to_string(), 3600);
    let _ = jwt.validate_token(&candidate);
});
