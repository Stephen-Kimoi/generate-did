use clap::Parser;
use generate_did::DidGenerator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The name of the canister (directory) to generate the .did file for
    canister_name: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let canister_name = cli.canister_name;

    // Check if current dir is the canister dir (contains Cargo.toml)
    let current_dir = std::env::current_dir()?;
    let current_cargo = current_dir.join("Cargo.toml");
    let is_canister_dir = current_cargo.exists() && current_dir.ends_with(&canister_name);

    let canister_path: PathBuf = if is_canister_dir {
        current_dir
    } else {
        // Try src/<canister_name>
        let candidate = current_dir.join("src").join(&canister_name);
        if candidate.join("Cargo.toml").exists() {
            candidate
        } else {
            anyhow::bail!("Could not find canister directory for '{}'. Run from the canister dir or project root.", canister_name);
        }
    };

    let generator = DidGenerator::new(canister_path);
    generator.generate()?;
    Ok(())
} 