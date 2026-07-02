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

//! Fuzz target: `ChaCha20Poly1305::decrypt`
//!
//! Strategy: ChaCha20-Poly1305 is the other half of the AEAD surface
//! that `RiCrypto` exposes (the first being AES-GCM, covered by
//! `crypto_decrypt`). A correct implementation rejects forgeries without
//! panicking, returning either `Ok(Vec<u8>)` (authenticated plaintext)
//! or `Err(_)` (rejected). A panic or allocator failure here is a
//! finding in the ring binding or the length-validation step.

use libfuzzer_sys::fuzz_target;
use ri::protocol::crypto::ChaCha20Poly1305;

fuzz_target!(|data: &[u8]| {
    if data.len() < 12 {
        return;
    }
    // The split is informational: ChaCha20-Poly1305 only requires a
    // 12-byte nonce prefix; the rest is the AEAD-sealed payload.
    let split = data.len() / 2;
    let (ct, aad) = data.split_at(split);
    if let Ok(cipher) = ChaCha20Poly1305::new() {
        let _ = cipher.decrypt(ct, Some(aad));
        // Also try a few well-known bad shapes.
        let _ = cipher.decrypt(&[], None);
        let _ = cipher.decrypt(&data[..1], None);
    }
});
