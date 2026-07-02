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

//! Fuzz target: `KyberKEM::decapsulate`
//!
//! Strategy: the ciphertext passed to `decapsulate` is fully
//! attacker-controlled. The secret key is a long-term private input but
//! we may receive it from disk or from a corrupted store. We exercise
//! three paths:
//!   1. Attacker ciphertext + empty secret key (length validation path).
//!   2. Attacker ciphertext + attacker "secret key" (corrupted-store path).
//!   3. Truncated / padded ciphertexts that oqs will reject — these must
//!      be rejected, not panicked over.

use libfuzzer_sys::fuzz_target;
use ri::protocol::kyber::KyberKEM;

fuzz_target!(|data: &[u8]| {
    let kem = KyberKEM::new();

    // Path 1: arbitrary ciphertext, empty secret key.
    let _ = kem.decapsulate(data, &[]);

    // Path 2: split the input into (ct, sk) and feed both as
    // attacker-controlled. The split point is itself attacker-influenced
    // because the fuzzer mutates the input buffer.
    if data.len() >= 2 {
        let split = (data[0] as usize).min(data.len() - 1);
        let (ct, sk) = data.split_at(split);
        let _ = kem.decapsulate(ct, &sk[..sk.len().min(4096)]);
    }

    // Path 3: known-bad lengths. oqs must return an error; a panic here
    // would be a finding in the integration layer.
    if data.len() > 1 {
        let _ = kem.decapsulate(&data[..1], &data[1..]);
    }
});
