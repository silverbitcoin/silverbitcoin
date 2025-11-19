//! # Quantum CLI
//!
//! Package manager and build tool for Quantum smart contracts.

mod commands;
mod dependency;
mod lockfile;
mod manifest;
mod package;
mod registry;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "quantum")]
#[command(about = "Quantum package manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Quantum package
    New {
        /// Package name
        name: String,
        /// Create in current directory
        #[arg(long)]
        here: bool,
    },
    /// Build the current package
    Build {
        /// Release mode
        #[arg(short, long)]
        release: bool,
        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Publish package to registry
    Publish {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Registry URL (defaults to official registry)
        #[arg(long)]
        registry: Option<String>,
    },
    /// Run tests
    Test {
        /// Filter tests by name
        filter: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, here } => {
            commands::new::execute(&name, here).await?;
        }
        Commands::Build { release, output } => {
            commands::build::execute(release, output.as_deref()).await?;
        }
        Commands::Publish { yes, registry } => {
            commands::publish::execute(yes, registry.as_deref()).await?;
        }
        Commands::Test { filter } => {
            commands::test::execute(filter.as_deref()).await?;
        }
    }

    Ok(())
}
