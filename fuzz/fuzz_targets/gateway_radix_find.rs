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

//! Fuzz target: `RadixTree::find` after `insert`
//!
//! Strategy: build a radix tree, register an attacker-chosen path as a
//! route, then look up another attacker-chosen path. This drives the
//! recursive `find_recursive` walker with combinations designed to:
//!   * exceed the recursion budget (e.g. a path with thousands of `/`),
//!   * collide with the route through wildcard/param boundaries,
//!   * alias a static path with a param (`/users/:id` vs `/users/me`).
//! The harness uses a no-op handler so we never actually invoke user code.

use std::pin::Pin;
use std::future::{ready, Future};
use libfuzzer_sys::fuzz_target;
use ri::gateway::RiRadixTree;
use ri::gateway::routing::RiRoute;
use ri::gateway::RiGatewayRequest;
use ri::gateway::RiGatewayResponse;
use ri::RiResult;

fn noop_handler(
    _req: RiGatewayRequest,
) -> Pin<Box<dyn Future<Output = RiResult<RiGatewayResponse>> + Send + Sync>> {
    Box::pin(ready(Ok(RiGatewayResponse::new(200, Vec::new(), "fuzz".to_string()))))
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    let split = (data[0] as usize) % data.len();
    let (register, lookup) = data.split_at(split);
    if register.is_empty() || lookup.is_empty() {
        return;
    }
    let register_path = String::from_utf8_lossy(register);
    let lookup_path = String::from_utf8_lossy(lookup);
    // Cap both sides to keep the recursion under control.
    if register_path.len() > 2048 || lookup_path.len() > 2048 {
        return;
    }

    let tree = RiRadixTree::new();
    let route = RiRoute::new(
        "GET".to_string(),
        register_path.into_owned(),
        std::sync::Arc::new(noop_handler),
    );
    tree.insert(route);
    let _ = tree.find(&lookup_path);
});
