# generate-did

[![Crates.io](https://img.shields.io/crates/v/generate-did.svg)](https://crates.io/crates/generate-did)
[![Docs.rs](https://docs.rs/generate-did/badge.svg)](https://docs.rs/generate-did)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Test Examples](https://github.com/Stephen-Kimoi/generate-did/actions/workflows/test-examples.yml/badge.svg)](https://github.com/Stephen-Kimoi/generate-did/actions/workflows/test-examples.yml)

A command-line tool to generate Candid (`.did`) files for Internet Computer Rust canisters.

## Features

- Build your Rust canister and extract its Candid interface automatically.
- Simple, robust CLI.
<!-- - Works from either the project root or the canister directory. -->
<!-- - Always places the `.did` file in the canister directory (DFX-compatible). -->

## Installation

Install from crates.io:
```bash
cargo install generate-did
```
<!-- Or, install from your local project:
```bash
cargo install --path .
``` -->

## Usage

From your project root **or** from inside the canister directory, run:
```bash
generate-did <canister_name>
```
- Replace `<canister_name>` with the directory name of your canister (must contain a `Cargo.toml`).
- The `.did` file will always be placed in the canister directory.

## Requirements

- [candid-extractor](https://github.com/dfinity/candid) must be installed and in your PATH:
  ```bash
  cargo install candid-extractor
  ```
- Ensure in your Rust canister code, you've called the `export_candid` macro at the end of your `lib.rs` file: 
```rust 
use ic_cdk::query;
use ic_cdk::update;

#[query]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn world(name: String) -> String {
    format!("World, {}!", name)
}

// Enable Candid export
ic_cdk::export_candid!();
```

- The canister must be a Rust project with a valid `Cargo.toml` and buildable to WASM.

## License

This project is licensed under the [MIT License](LICENSE).
