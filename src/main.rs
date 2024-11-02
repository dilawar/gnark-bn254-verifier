use std::path::PathBuf;

use clap::Parser;
use gnark_bn254_verifier::{verify, Fr, ProvingSystem};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the proof
    proof: PathBuf,

    /// Verifying key
    verifying_key: PathBuf,

    /// Verifying key
    proving_system: Option<PrvSystem>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let proof = std::fs::read(cli.proof)?;
    let vk = std::fs::read(cli.verifying_key)?;
    let method = cli.proving_system.unwrap_or_default();

    if verify(&proof, &vk, &[Fr::from(1u8), Fr::from(7u8)], method.into()) {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }

    Ok(())
}

#[derive(clap::ValueEnum, Default, Clone)]
enum PrvSystem {
    #[default]
    Groth16,
    Plonk,
}

impl From<PrvSystem> for ProvingSystem {
    fn from(psystem: PrvSystem) -> ProvingSystem {
        match psystem {
            PrvSystem::Groth16 => ProvingSystem::Groth16,
            PrvSystem::Plonk => ProvingSystem::Plonk,
        }
    }
}
