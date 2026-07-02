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

//! Fuzz target: `RiCachedValue::deserialize`
//!
//! Strategy: a cache value's `value` field may originate from a remote
//! Redis or other backend, and is deserialized as JSON on every cache hit.
//! An attacker who can poison the cache backend (or who controls a peer
//! that shares a cache namespace) can drive this path. The target wraps
//! arbitrary bytes as the JSON payload and lets the deserializer chew on
//! them.

use libfuzzer_sys::fuzz_target;
use ri::cache::core::RiCachedValue;
use serde::Deserialize;

#[derive(Deserialize)]
struct DummyTarget {
    #[allow(dead_code)]
    a: u32,
    #[allow(dead_code)]
    b: String,
}

fuzz_target!(|data: &[u8]| {
    let payload = String::from_utf8_lossy(data);
    let entry = RiCachedValue::new(payload.into_owned(), Some(60));
    let _: Result<DummyTarget, _> = entry.deserialize();
});
