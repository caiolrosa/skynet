use anyhow::{Context, Result};
use bob::agent_manager::{AgentManager, Provider};
use bob::agent_runner::AgentRunner;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bob")]
#[command(about = "Bob CLI - AI Agent Orchestrator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bootstrap a project from a Markdown spec
    Init {
        /// Spec file path
        #[arg(short, long)]
        file: String,

        /// Agent provider
        #[arg(long, value_enum)]
        provider: Provider,
    },
    /// Execute the agent-driven development loop
    Run {
        /// Agent provider
        #[arg(long, value_enum)]
        provider: Provider,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner = AgentRunner;
    let manager = AgentManager::new(runner);

    match &cli.command {
        Commands::Init { file, provider } => {
            manager.init(file, provider).context("Error during init")?;
        }
        Commands::Run { provider } => {
            manager.run(provider).context("Error during run")?;
        }
    }

    Ok(())
}
