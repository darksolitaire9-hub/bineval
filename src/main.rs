pub mod core;
pub mod ports;
pub mod adapters;

use clap::{Parser, Subcommand};
use ports::{CreditsPort, OutputPort};
use adapters::credits::CreditsAdapter;
use adapters::cli_output::CliOutputAdapter;

#[derive(Parser)]
#[command(name = "bineval")]
#[command(about = "A Rust-native evaluation kernel and suite system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run discovery and evaluation over primitives
    Audit,
    /// Run an evaluation suite
    RunSuite {
        name: String,
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Print credits for the project or specific entity
    Credits {
        #[arg(long)]
        primitive: Option<String>,
        #[arg(long)]
        suite: Option<String>,
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let output = CliOutputAdapter;

    match &cli.command {
        Commands::Audit => {
            // Stub for audit command, but we would use tokio here
            output.print_report("--- PRIMITIVES ---\n\nAUDIT: OK")?;
        }
        Commands::RunSuite { name, target, json } => {
            // Stub for run suite
            output.print_report(&format!("Running suite: {}", name))?;
            if let Some(t) = target {
                output.print_report(&format!("Target: {}", t))?;
            }
            if *json {
                // In the future this prints the aggregated JSON summary
            }
        }
        Commands::Credits { primitive, suite, json } => {
            let adapter = CreditsAdapter;
            let credits = if let Some(p) = primitive {
                adapter.load_entity_credits(p)?
            } else if let Some(s) = suite {
                adapter.load_entity_credits(s)?
            } else {
                adapter.load_project_credits(".")?
            };

            if *json {
                let val = serde_json::to_value(&credits)?;
                println!("{}", serde_json::to_string_pretty(&val)?);
            } else {
                output.print_report("Credits:")?;
                for h in credits.humans {
                    output.print_report(&format!("  {} ({})", h.name, h.role))?;
                }
                for m in credits.models {
                    output.print_report(&format!("  {} ({})", m.model_name, m.purpose))?;
                }
            }
        }
    }

    Ok(())
}
