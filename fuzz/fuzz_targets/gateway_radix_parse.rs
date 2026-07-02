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

//! Fuzz target: `RadixTree::parse_path`
//!
//! Strategy: the parser splits on `/` and wraps each non-empty fragment in
//! a `PathSegment`. The first byte determines the segment type
//! (`*` => wildcard, `:` => param, otherwise static). Bug surface includes:
//!   * empty segments after `split('/').filter(!is_empty)` — the filter
//!     drops them, but a trailing `/` or double `//` should never trigger
//!     any other branch,
//!   * `PathSegment::new` does not perform validation, so the result of an
//!     attacker-controlled path becomes the *route key* in the radix tree.
//!     Length-based DoS (a path of 1 GiB of slashes) must be capped before
//!     it gets here in production, but `parse_path` is the last line of
//!     defense.

use libfuzzer_sys::fuzz_target;
use ri::gateway::RiRadixTree;

fuzz_target!(|data: &[u8]| {
    // 16 KiB cap keeps the harness from spending all of its time on a
    // single huge allocation; production has its own upstream cap.
    if data.len() > 16 * 1024 {
        return;
    }
    let path = String::from_utf8_lossy(data);
    let _ = RiRadixTree::parse_path(&path);
});
