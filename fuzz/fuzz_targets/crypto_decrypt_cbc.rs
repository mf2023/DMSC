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

//! Fuzz target: AES-CBC decrypt (legacy path)
//!
//! Strategy: the CBC mode path is exactly where padding-oracle and
//! length-confusion bugs tend to live. We feed arbitrary ciphertexts
//! (which the production code must validate as a 16-byte multiple and
//! reject otherwise) to ensure the early-return guards cannot be
//! bypassed. Padding validation is *not* implemented in the current
//! decrypt_cbc (the responsibility is deferred to the caller), so this
//! target is most useful for spotting panics on malformed lengths.

use libfuzzer_sys::fuzz_target;
use ri::protocol::crypto::RiCrypto;

fuzz_target!(|data: &[u8]| {
    if let Ok(cipher) = RiCrypto::new() {
        let _ = cipher.decrypt_cbc(data);
    }
});
