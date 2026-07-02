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

//! Fuzz target: AES-GCM authenticated decrypt
//!
//! Strategy: build a fresh cipher, then feed arbitrary bytes as
//! `ciphertext || additional_data`. A correct implementation must reject
//! forgeries (ring returns a `Unspecified` error) without panicking or
//! leaking memory. This guards the *integration* (length checks, error
//! mapping) rather than the ring primitive itself.

use libfuzzer_sys::fuzz_target;
use ri::protocol::crypto::RiCrypto;

fuzz_target!(|data: &[u8]| {
    // The split is a soft heuristic; the fuzzer will explore every cut.
    if data.len() < 12 {
        return;
    }
    let split = data.len() / 2;
    let (ciphertext, aad) = data.split_at(split);
    if let Ok(cipher) = RiCrypto::new() {
        let _ = cipher.decrypt(ciphertext, Some(aad));
    }
});
