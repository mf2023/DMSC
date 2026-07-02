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

//! Fuzz target: Ed25519 signature verification
//!
//! Strategy: take arbitrary bytes, split them into (public_key, message,
//! signature) and call `verify_ed25519`. Any panic or OOB read inside the
//! integration layer is a finding. The real `ring` verifier is trusted;
//! we are auditing the way Ri passes data into it.

use libfuzzer_sys::fuzz_target;
use ri::protocol::crypto::RiCrypto;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 + 64 {
        return;
    }
    // Ed25519 public key is 32 bytes, signature is 64 bytes; the rest
    // is treated as the message.
    let pk = &data[0..32];
    let sig = &data[32..96];
    let msg = &data[96..];
    if let Ok(cipher) = RiCrypto::new() {
        let _ = cipher.verify_ed25519(msg, sig, pk);
    }
});
