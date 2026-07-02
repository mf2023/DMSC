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

//! Fuzz target: `RiValidatorBuilder::is_email`
//!
//! Strategy: the email rule is a regex match against a value that may
//! arrive from a user registration form, a webhook payload, or a
//! configuration file. The value space is unbounded. The harness caps
//! the input size so a single iteration cannot exhaust the heap, but
//! otherwise exercises every possible character sequence.

use libfuzzer_sys::fuzz_target;
use ri::validation::RiValidatorBuilder;

fuzz_target!(|data: &[u8]| {
    if data.len() > 2048 {
        return;
    }
    let value = String::from_utf8_lossy(data);
    let runner = RiValidatorBuilder::new("email").is_email().build();
    let _ = runner.validate_value(Some(&value));
});
