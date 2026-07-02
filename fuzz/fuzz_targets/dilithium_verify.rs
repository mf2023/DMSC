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

//! Fuzz target: `DilithiumSigner::verify`
//!
//! Strategy: the public-key, message, and signature triple is the entire
//! attack surface for a post-quantum signature scheme. This target
//! generates a real Dilithium2 key pair once via `OnceLock` (avoiding the
//! cost of keygen on every iteration) and then feeds attacker-controlled
//! (public_key, message, signature) triples into `verify`.
//!
//! Dilithium2 sizes: pk = 1312 B, signature = 2420 B. Inputs of the wrong
//! length are rejected by the underlying oqs library; the fuzzer is
//! expected to discover any pre-validation path that *panics* instead of
//! returning an error.

use std::sync::OnceLock;
use libfuzzer_sys::fuzz_target;
use ri::protocol::dilithium::DilithiumSigner;

static KEYPAIR: OnceLock<(Vec<u8>, Vec<u8>)> = OnceLock::new();

fuzz_target!(|data: &[u8]| {
    let (pk, sk) = KEYPAIR
        .get_or_init(|| {
            let signer = DilithiumSigner::new();
            signer.keygen().expect("keygen should succeed under fuzz")
        })
        .clone();

    // Split the input into (pk, msg, sig). Any remainder is the message.
    let pk_part = if data.len() >= pk.len() { &data[..pk.len()] } else { data };
    let sig_part = if data.len() >= 2420 {
        &data[pk_part.len()..pk_part.len() + 2420]
    } else {
        &data[pk_part.len()..]
    };
    let msg_start = pk_part.len() + sig_part.len();
    let msg = &data[msg_start..];

    let signer = DilithiumSigner::new();
    // The interesting result is a returned `Ok(false)` (signature rejected)
    // or `Ok(true)` (signature accepted). Any panic or `unwrap` failure is
    // a finding.
    let _ = signer.verify(pk_part, msg, sig_part);
    // Also exercise the API with a real key so the integration path runs
    // end-to-end against attacker-chosen data.
    let _ = signer.verify(&pk, msg, &[]);
    let _ = signer.verify(&pk, b"", sig_part);
    let _ = signer.verify(&pk, b"x", &sk[..sk.len().min(sig_part.len())]);
});
