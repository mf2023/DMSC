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

//! Fuzz target: `PreparedStatement::new`
//!
//! Strategy: SQL parameter extraction is a common parser bug surface.
//! `extract_params` must handle pathological inputs (deeply nested
//! parentheses, multibyte UTF-8 fragments, NUL bytes, very long
//! identifiers) without panicking. The target is also useful for
//! spotting accidental SQL-injection sinks where a raw string is later
//! concatenated into a query.

use libfuzzer_sys::fuzz_target;
use ri::database::statement::PreparedStatement;

fuzz_target!(|data: &[u8]| {
    let sql = String::from_utf8_lossy(data);
    let stmt = PreparedStatement::new(&sql);
    // Touch the extracted parameters to make sure the parsing result
    // is exercised and the lazy view doesn't hide a panic.
    let _ = stmt.param_count();
});
