# Ri Fuzzing

This directory hosts the fuzz targets used to surface security and stability
issues in the Ri project. Targets are run with [`cargo-fuzz`][cf] (powered by
`libFuzzer`).

The fuzzing workspace is intentionally separated from the main `Cargo.toml`
to keep the `libfuzzer-sys` and `arbitrary` dependencies out of production
builds.

## Prerequisites

Install the `cargo-fuzz` subcommand and a nightly toolchain (the fuzzer
requires nightly):

```sh
rustup install nightly
cargo install cargo-fuzz
```

The project must already build (`cargo build`) on the target host because
the fuzz binaries link against `ri`.

## Running a target

From the repository root:

```sh
cargo +nightly fuzz run protocol_frame_parse
```

Useful flags:

| Flag                                | Effect                                            |
| ----------------------------------- | ------------------------------------------------- |
| `-- -max_total_time=300`            | Run for 5 minutes then stop (good for CI smoke)   |
| `-- -max_len=4096`                  | Cap input size to 4 KiB (faster corpus growth)    |
| `-- -jobs=4 -workers=4`             | Parallel fuzzing on 4 cores                       |
| `-- <target> <crash-file>`          | Reproduce a single crash input                    |

## Targets

| Target                              | What it exercises                                       |
| ----------------------------------- | ------------------------------------------------------- |
| `protocol_frame_parse`              | `RiFrame::from_bytes` on arbitrary wire data            |
| `protocol_frame_header_parse`       | `RiFrameHeader::from_bytes` (32-byte header parser)     |
| `jwt_validate`                      | `RiJwt::validate_token` on attacker-controlled strings  |
| `cache_deserialize`                 | `RiCachedValue::deserialize` (JSON over arbitrary bytes)|
| `prepared_statement_new`            | `PreparedStatement::new` SQL parameter extraction       |
| `crypto_decrypt`                    | AES-GCM authenticated decrypt integration               |
| `crypto_verify_ed25519`             | Ed25519 signature verification integration              |
| `crypto_decrypt_cbc`                | AES-CBC decrypt path (length validation)                |
| `queue_message_headers`             | `RiQueueMessage::with_headers` (UTF-8 boundary panic)   |
| `gateway_radix_parse`               | `RadixTree::parse_path` (route segment parser)           |
| `gateway_radix_find`                | `RadixTree::insert` + `find` (recursive walker)         |
| `dilithium_verify`                  | `DilithiumSigner::verify` (PQC signature integration)   |
| `kyber_decapsulate`                 | `KyberKEM::decapsulate` (PQC KEM integration)           |
| `observability_trace_context`       | `RiTraceContext::from_header` (W3C traceparent)         |
| `observability_baggage`             | `RiBaggage::from_header` (W3C baggage)                  |
| `serialization_json`                | `serde_json::from_slice` (deep-nest / allocation bomb)  |
| `serialization_yaml`                | `serde_yaml::from_slice` (anchor / merge-key bomb)      |
| `serialization_bincode`             | `bincode::deserialize` (length-prefix DoS)              |
| `validation_url`                    | `RiValidatorBuilder::is_url` (URL parse edge cases)     |
| `validation_uuid`                   | `RiValidatorBuilder::is_uuid` (regex edge cases)         |
| `validation_email`                  | `RiValidatorBuilder::is_email` (regex backtracking)     |
| `crypto_chacha20poly1305`           | `ChaCha20Poly1305::decrypt` (AEAD forgery rejection)    |
| `crypto_verify_ecdsa`               | `ECDSAVerifier::verify` (P-256 point/scalar validation) |
| `regex_match`                       | arbitrary pattern + input (ReDoS detection)             |
| `uuid_parse`                        | `Uuid::try_parse` / `try_from_slice` (edge forms)       |

## Continuous integration

The CI workflow runs a short (60 second) smoke test on every target when
the `fuzz` label is applied to a pull request. Long-running nightly fuzz
campaigns are run on a separate schedule and any crash artifacts are
uploaded as workflow artifacts.

[cf]: https://github.com/rust-fuzz/cargo-fuzz
