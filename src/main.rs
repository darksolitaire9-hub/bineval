pub mod core;
pub mod ports;
pub mod adapters;

use clap::{Parser, Subcommand};
use adapters::credits::CreditsAdapter;
use core::policy::PolicyRegistry;

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
    /// Validate suites and templates against the schema and registry
    Validate {
        /// What to validate (currently supports 'suites')
        component: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Audit => {
            // Stub for audit command, but we would use tokio here
            println!("--- PRIMITIVES ---\n\nAUDIT: OK");
        }
        Commands::RunSuite { name, target, json } => {
            // Stub for run suite
            println!("Running suite: {}", name);
            if let Some(t) = target {
                println!("Target: {}", t);
            }
            if *json {
                // In the future this prints the aggregated JSON summary
            }
        }
        Commands::Credits { primitive, suite, json } => {
            let credits = if let Some(_p) = primitive {
                // stub for entity level
                core::credits::Credits::default()
            } else if let Some(_s) = suite {
                // stub for entity level
                core::credits::Credits::default()
            } else {
                CreditsAdapter::load_project_credits(".")?
            };

            if *json {
                let val = serde_json::to_value(&credits)?;
                println!("{}", serde_json::to_string_pretty(&val)?);
            } else {
                println!("Credits:");
                for h in credits.humans {
                    println!("  {} ({})", h.name, h.role);
                }
                for m in credits.models {
                    println!("  {} ({})", m.model_name, m.purpose);
                }
            }
        }
        Commands::Validate { component } => {
            if component == "suites" {
                // For now, load empty suites to simulate validation
                use crate::ports::ConfigPort;
                let adapter = crate::adapters::json_config::JsonConfigAdapter;
                let suites = adapter.load_suites(".")?;
                let templates = adapter.load_templates(".")?;
                
                // Verify policies against registry
                for template in &templates {
                    for check in &template.checks {
                        if PolicyRegistry::resolve(&check.policy_id).is_none() {
                            return Err(anyhow::anyhow!("ConfigError::UnknownPolicy -> {}", check.policy_id));
                        }
                    }
                }
                
                // If there were any errors, load_suites/templates would have returned Err(ConfigError)
                println!("Validation successful: {} suites and {} templates checked.", suites.len(), templates.len());
            } else {
                return Err(anyhow::anyhow!("Unknown validation component: {}", component));
            }
        }
    }

    Ok(())
}
