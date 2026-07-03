use clap::{Parser, Subcommand};
use std::path::PathBuf;

use bineval::adapters::fs::primitives::FsPrimitiveRepositoryAdapter;
use bineval::adapters::fs::suites::FsSuiteRepositoryAdapter;
use bineval::adapters::logging::stdout_logger::StdoutLogger;
use bineval::core::ports::LoggerPort;
use bineval::core::usecases::audit::AuditUseCase;
use bineval::core::usecases::run_suite::RunSuiteUseCase;
use bineval::core::usecases::validate::ValidateUseCase;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit all primitives against all suites
    Audit {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = ".")]
        targets: PathBuf,
    },
    /// Run a specific suite
    Run {
        #[arg(short, long)]
        suite: String,
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = ".")]
        targets: PathBuf,
    },
    /// Validate that suites can be parsed
    Validate {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let logger = StdoutLogger;

    match &cli.command {
        Commands::Audit { path, targets } => {
            let primitives_repo = FsPrimitiveRepositoryAdapter::new(targets);
            let suites_repo = FsSuiteRepositoryAdapter::new(path);
            let use_case = AuditUseCase {
                primitives: &primitives_repo,
                suites: &suites_repo,
                logger: &logger,
            };

            match use_case.run() {
                Ok(result) => {
                    if result.passed {
                        logger.info("AUDIT PASSED: All policies succeeded.");
                        std::process::exit(0);
                    } else {
                        logger.error("AUDIT FAILED: One or more policies failed.");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    logger.error(&format!("AUDIT ERROR: {}", e));
                    std::process::exit(1);
                }
            }
        }
        Commands::Run {
            suite,
            path,
            targets,
        } => {
            let primitives_repo = FsPrimitiveRepositoryAdapter::new(targets);
            let suites_repo = FsSuiteRepositoryAdapter::new(path);
            let use_case = RunSuiteUseCase {
                primitives: &primitives_repo,
                suites: &suites_repo,
                logger: &logger,
            };

            match use_case.run_suite(suite) {
                Ok(result) => {
                    if result.passed {
                        logger.info(&format!("SUITE '{}' PASSED", suite));
                        std::process::exit(0);
                    } else {
                        logger.error(&format!("SUITE '{}' FAILED", suite));
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    logger.error(&format!("RUN ERROR: {}", e));
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate { path } => {
            let suites_repo = FsSuiteRepositoryAdapter::new(path);
            let use_case = ValidateUseCase {
                suites: &suites_repo,
                logger: &logger,
            };

            match use_case.run() {
                Ok(_) => {
                    logger.info("VALID: suites OK");
                    std::process::exit(0);
                }
                Err(e) => {
                    logger.error(&format!("VALIDATION ERROR: {}", e));
                    std::process::exit(1);
                }
            }
        }
    }
}
