//!
//! # generate-did
//!
//! `generate-did` is a CLI tool to generate Candid (`.did`) files for Internet Computer Rust canisters.
//!
//! See the [README](https://github.com/Stephen-Kimoi/generate-did) for CLI usage and installation instructions.
//!
//! ## Example
//!
//! ```sh
//! generate-did <canister_name>
//! ```

use std::process::Command;
use std::path::PathBuf;
use anyhow::{Result, Context};
use thiserror::Error;

/// Errors that can occur during DID generation.
#[derive(Error, Debug)]
pub enum DidGeneratorError {
    #[error("Failed to build canister: {0}")]
    BuildError(String),
    #[error("Failed to generate Candid file: {0}")]
    CandidGenerationError(String),
    #[error("Failed to write .did file: {0}")]
    FileWriteError(String),
}

/// A struct for generating Candid (.did) files for Internet Computer canisters.
///
/// Most users should use the CLI (`generate-did <canister_name>`) instead of this struct directly.
pub struct DidGenerator {
    canister_dir: PathBuf,
    canister_name: String,
}

impl DidGenerator {
    /// Creates a new DidGenerator instance.
    ///
    /// # Arguments
    ///
    /// * `canister_dir` - The path to the canister directory
    pub fn new(canister_dir: PathBuf) -> Self {
        let canister_name = canister_dir.file_name().unwrap().to_string_lossy().to_string();
        Self {
            canister_dir,
            canister_name,
        }
    }

    /// Generates the .did file for the specified canister.
    ///
    /// This function:
    /// 1. Builds the Rust canister
    /// 2. Extracts the Candid interface using candid-extractor
    /// 3. Writes the interface to a .did file
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok(()) if successful, Err if any step fails
    pub fn generate(&self) -> Result<()> {
        println!("Generating .did file for canister: {}...", self.canister_name);

        let wasm_path = self.canister_dir.join("target/wasm32-unknown-unknown/release").join(format!("{}.wasm", self.canister_name));
        let did_path = self.canister_dir.join(format!("{}.did", self.canister_name));

        // Build the Rust canister
        let build_status = Command::new("cargo")
            .current_dir(&self.canister_dir)
            .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
            .status()
            .context("Failed to execute cargo build command")?;

        if !build_status.success() {
            return Err(DidGeneratorError::BuildError(
                "Failed to build canister".to_string(),
            ).into());
        }

        // Verify the WASM file exists
        if !wasm_path.exists() {
            return Err(DidGeneratorError::BuildError(
                format!("WASM file not found at: {}", wasm_path.display())
            ).into());
        }

        // Generate the Candid file
        let output = Command::new("candid-extractor")
            .arg(&wasm_path)
            .output()
            .context("Failed to execute candid-extractor")?;

        if !output.status.success() {
            return Err(DidGeneratorError::CandidGenerationError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ).into());
        }

        // Write the output to the .did file
        std::fs::write(&did_path, output.stdout)
            .context(format!("Failed to write .did file to {}", did_path.display()))?;

        println!(
            "Candid file generated successfully: {}",
            did_path.display()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    // Helper macro for cleanup (only used in tests)
    macro_rules! defer {
        ($e:expr) => {
            let _defer = Defer(Some(|| { let _ = $e; }));
        };
    }
    struct Defer<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f();
            }
        }
    }

    fn setup_test_environment() -> Result<()> {
        // Ensure the test canister directory exists
        let test_canister_dir = Path::new("src/test_canister");
        if !test_canister_dir.exists() {
            fs::create_dir_all(test_canister_dir)?;
        }
        Ok(())
    }

    fn cleanup_test_environment() -> Result<()> {
        // Clean up generated files
        let did_file = Path::new("src/test_canister/test_canister.did");
        if did_file.exists() {
            fs::remove_file(did_file)?;
        }
        Ok(())
    }

    #[test]
    fn test_did_generator_creation() {
        let generator = DidGenerator::new("test_canister".into());
        assert_eq!(generator.canister_name, "test_canister");
    }

    #[test]
    fn test_did_generation() -> Result<()> {
        setup_test_environment()?;
        defer!(cleanup_test_environment());

        let generator = DidGenerator::new("test_canister".into());
        generator.generate()?;

        // Verify that the .did file was created
        let did_path = Path::new("src/test_canister/test_canister.did");
        assert!(did_path.exists(), "DID file was not created");

        // Read and verify the content of the .did file
        let did_content = fs::read_to_string(did_path)?;
        assert!(!did_content.is_empty(), "DID file is empty");
        assert!(did_content.contains("type User"), "DID file should contain User type");
        assert!(did_content.contains("service"), "DID file should contain service definition");

        Ok(())
    }
}