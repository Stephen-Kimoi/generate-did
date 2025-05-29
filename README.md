# generate-did

[![Crates.io](https://img.shields.io/crates/v/generate-did.svg)](https://crates.io/crates/generate-did)
[![Docs.rs](https://docs.rs/generate-did/badge.svg)](https://docs.rs/generate-did)

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
- The canister must be a Rust project with a valid `Cargo.toml` and buildable to WASM.

## License

This project is licensed under the [MIT License](LICENSE).
