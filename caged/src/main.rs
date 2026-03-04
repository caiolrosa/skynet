mod cli;
mod config;
mod docker;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use docker::DockerOrchestrator;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = Config::find_config(cli.file)?;
    let config = Config::load(&config_path)?;
    let orchestrator = DockerOrchestrator::new()?;

    match cli.command {
        Commands::Run => {
            if !orchestrator.image_exists()? {
                orchestrator.build_image(&config)?;
            }

            orchestrator.run_agent(&config)?;
        }
        Commands::Shell => {
            if !orchestrator.image_exists()? {
                orchestrator.build_image(&config)?;
            }
            orchestrator.shell(&config)?;
        }
        Commands::Cleanup => {
            orchestrator.cleanup()?;
        }
        Commands::Build => {
            orchestrator.build_image(&config)?;
        }
    }

    Ok(())
}
