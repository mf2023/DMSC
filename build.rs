// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Build script: generate the C API header (include/ri.h) on every
// `cargo build`, so the header is produced during normal compilation and
// not only when the release-time `make build-c` step runs.

use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("cbindgen.toml");
    let include_dir = manifest_dir.join("include");
    let header_path = include_dir.join("ri.h");

    std::fs::create_dir_all(&include_dir)
        .expect("Failed to create include/ directory for generated C header");

    let bindings = cbindgen::Builder::new()
        .with_crate(&manifest_dir)
        .with_config(&config_path)
        .generate()
        .expect("Failed to generate C bindings (include/ri.h) via cbindgen");

    bindings
        .write_to_file(&header_path)
        .expect("Failed to write include/ri.h");

    // Re-run this build script when the C API sources or cbindgen config change.
    println!("cargo:rerun-if-changed=src/c/mod.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}
