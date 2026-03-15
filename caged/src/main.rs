use anyhow::Result;
use clap::Parser;
use caged::cli::{Cli, Commands};
use caged::config::Config;
use caged::docker::DockerOrchestrator;
use caged::docker_runner::DockerRunner;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = Config::find_config(cli.file)?;
    let config = Config::load(&config_path)?;
    
    let runner = DockerRunner::new();
    let orchestrator = DockerOrchestrator::new(config_path, runner)?;

    match cli.command {
        Commands::Run => {
            if !orchestrator.image_exists()? {
                orchestrator.build_image(&config)?;
            }

            orchestrator.run_agent(&config)?;
        }
        Commands::Shell { command } => {
            if !orchestrator.image_exists()? {
                orchestrator.build_image(&config)?;
            }
            orchestrator.shell(&config, command)?;
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
