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

//! Fuzz target: `RiValidatorBuilder::is_url`
//!
//! Strategy: a remote URL value may arrive from a config file, a CLI
//! argument, an HTTP redirect target, or a service-mesh registry entry.
//! `is_url` is a thin wrapper around `url::Url::parse`, but the
//! validation pipeline still touches the value through `EMAIL_REGEX`
//! and friends, so we drive it through the public Ri API rather than
//! directly through `url` to surface any panic that the validation
//! integration might add on top of the parser.

use libfuzzer_sys::fuzz_target;
use ri::validation::RiValidatorBuilder;

fuzz_target!(|data: &[u8]| {
    if data.len() > 4096 {
        return;
    }
    let value = String::from_utf8_lossy(data);
    let runner = RiValidatorBuilder::new("url").is_url().build();
    // validate_value is total; the result is discarded because we only
    // care about whether the call panics.
    let _ = runner.validate_value(Some(&value));
    // And exercise the `None` path so the optional/nullable guards are
    // hit too.
    let _ = runner.validate_value(None);
});
