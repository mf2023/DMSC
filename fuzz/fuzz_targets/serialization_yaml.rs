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

//! Fuzz target: `serde_yaml::from_slice`
//!
//! Strategy: YAML has a long history of denial-of-service vectors:
//!   * the billion-laughs anchor/alias bomb,
//!   * deeply nested mappings that overflow the stack,
//!   * strings that look like they declare enormous merge keys.
//! Any of these arriving through an attacker-controlled config file is a
//! finding.

use libfuzzer_sys::fuzz_target;
use serde_yaml::Value;

fuzz_target!(|data: &[u8]| {
    if data.len() > 64 * 1024 {
        return;
    }
    let _ = serde_yaml::from_slice::<Value>(data);
});
