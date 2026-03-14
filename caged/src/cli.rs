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
    Shell {
        /// Optional command and arguments to execute within the container
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// Forces a rebuild of the project-specific agent image
    Build,

    /// Stops and removes any containers and images associated with the current project directory
    Cleanup,
}
