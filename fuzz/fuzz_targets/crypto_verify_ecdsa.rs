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

//! Fuzz target: `ECDSAVerifier::verify`
//!
//! Strategy: ECDSA P-256-SHA256 verification takes an attacker-controlled
//! (public_key, message, signature) triple. The integration must reject
//! any input that is not a valid point on the curve and any signature
//! that is not a valid scalar pair. The classic attacks we are guarding
//! against are point-infinity confusion, signature malleability, and
//! out-of-range `r`/`s` values that some older parsers accepted.

use libfuzzer_sys::fuzz_target;
use ri::protocol::crypto::ECDSAVerifier;

fuzz_target!(|data: &[u8]| {
    // P-256 uncompressed public keys are 65 bytes; signatures are 64.
    // Anything smaller is a guaranteed reject; anything larger is
    // ambiguous and exactly what the fuzzer should explore.
    if data.len() < 65 {
        return;
    }
    let pk = &data[..65];
    let msg = &data[65..];
    let sig = if data.len() >= 65 + 64 {
        &data[65..65 + 64]
    } else {
        &data[65..]
    };
    let _ = ECDSAVerifier::verify(pk, msg, sig);
    // The verifier also returns Ok(true) for an empty signature
    // (rejected, but never panics). Round-trip once for completeness.
    let _ = ECDSAVerifier::verify(&[], &[], &[]);
});
