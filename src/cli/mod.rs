use crate::config::Config;
use crate::{jetbrains, mobaxterm};
use clap::{Parser, Subcommand};

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Mobaxterm error: {0}")]
    Mobaxterm(String),
    #[error("Jetbrains error: {0}")]
    Jetbrains(String),
}

/// 不可描述的功能
#[derive(Debug, Parser)]
#[command(
  version,
  about,
  long_about = None,
  disable_help_subcommand = true,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// MobXterm
    Mobaxterm,
    /// JetBrains
    Jetbrains,
}

pub async fn run(config: &Config) -> Result<(), CliError> {
    let args = Cli::parse();
    match args.command {
        Command::Mobaxterm => {
            mobaxterm::run(&config.mobaxterm).map_err(|e| CliError::Mobaxterm(e.to_string()))?;
        }
        Command::Jetbrains => {
            jetbrains::run(&config.jetbrains)
                .await
                .map_err(|e| CliError::Jetbrains(e.to_string()))?;
        }
    };
    Ok(())
}
