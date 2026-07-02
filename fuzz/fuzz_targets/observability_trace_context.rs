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

//! Fuzz target: `RiTraceContext::from_header` (W3C Trace Context)
//!
//! Strategy: W3C Trace Context is propagated as a request header and may
//! arrive from any upstream service, including ones outside the trust
//! boundary. The parser is required to return `None` on malformed input
//! instead of panicking. A panic here would let an attacker kill a single
//! worker thread by sending a hostile `traceparent` header.

use libfuzzer_sys::fuzz_target;
use ri::observability::propagation::RiTraceContext;

fuzz_target!(|data: &[u8]| {
    if data.len() > 4096 {
        return;
    }
    let header = String::from_utf8_lossy(data);
    // Exercise the full hex-parse path on every input. The fuzzer will
    // quickly discover inputs that pass the `len == 4` gate and break
    // the hex decoder.
    let _ = RiTraceContext::from_header(&header);
    // Also exercise the round-trip when parsing succeeds.
    if let Some(ctx) = RiTraceContext::from_header(&header) {
        // to_header must not panic for any valid context.
        let _ = ctx.to_header();
    }
});
