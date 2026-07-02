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

//! Fuzz target: `RiValidatorBuilder::is_uuid`
//!
//! Strategy: the UUID rule wraps a regex match. Regexes can panic if
//! driven past the `D` cap and can also exhibit catastrophic backtracking
//! on certain malformed inputs. Driving the validator through the public
//! API exercises the regex with every possible byte sequence the regex
//! crate will accept, and any panic becomes a finding.

use libfuzzer_sys::fuzz_target;
use ri::validation::RiValidatorBuilder;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1024 {
        return;
    }
    let value = String::from_utf8_lossy(data);
    let runner = RiValidatorBuilder::new("uuid").is_uuid().build();
    let _ = runner.validate_value(Some(&value));
});
