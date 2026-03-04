use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "caged")]
#[command(version, about = "Isolated Agent Execution", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Specifies a custom configuration file path (defaults to caged.yaml)
    #[arg(short, long, global = true)]
    pub file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Executes the agent specified in the configuration file
    Run,

    /// Starts an interactive bash shell session within the container
    Shell,

    /// Forces a rebuild of the project-specific agent image
    Build,

    /// Stops and removes any containers and images associated with the current project directory
    Cleanup,
}
