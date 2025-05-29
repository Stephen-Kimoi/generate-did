# generate-did

[![Crates.io](https://img.shields.io/crates/v/generate-did.svg)](https://crates.io/crates/generate-did)
[![Docs.rs](https://docs.rs/generate-did/badge.svg)](https://docs.rs/generate-did)

A tool to generate Candid (`.did`) files for Internet Computer Rust canisters.

## Features

- Build your Rust canister and extract its Candid interface automatically.
- Usable as a library in other Rust projects.
- Simple, type-safe API.

## Usage

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
generate-did = "0.1.0-beta.1"
```

Example:

```rust
use generate_did::DidGenerator;

fn main() -> anyhow::Result<()> {
    let generator = DidGenerator::new("your_canister_name");
    generator.generate()?;
    Ok(())
}
```

- Replace `"your_canister_name"` with the name of your canister directory (must contain a `Cargo.toml`).

### Requirements

- [candid-extractor](https://github.com/dfinity/candid) must be installed and in your PATH:
  ```bash
  cargo install candid-extractor
  ```
- The canister must be a Rust project with a valid `Cargo.toml` and buildable to WASM.

## Development

Clone the repo and run tests:

```bash
cargo test
```

## License

MIT
