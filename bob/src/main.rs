use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

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

#[derive(Clone, ValueEnum, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Provider {
    Gemini,
    Claude,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorklistItem {
    id: String,
    completed: bool,
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Worklist {
    spec_file_path: String,
    items: Vec<WorklistItem>,
}

const RUN_PROMPT_TEMPLATE: &str = r#"You are an autonomous coding agent working on a software project.
Follow the steps below strictly, do not make assumptions or guesses
when in doubt ask the user

- Read the spec at {{spec_file_path}}
- Read the worklist.json file in the current working directory
- Pay special attention to any notes (found in the notes field in worklist.json)
- Find the last non completed requirement (check completed field in worklist)
- Implement that single requirement
- Ensure test, linters and any other quality checks defined in the project's AGENTS.md are passing
- Update the requirement in the worklist.json file to set completed: true for the requirement just implemented
- Add any relevant notes to the notes field in worklist.json for the requirement just implemented

IMPORTANT: when all the above is completed respond with BOB_COMPLETED and stop
"#;

const INIT_PROMPT_TEMPLATE: &str = r#"
Your job is ONLY to convert the spec md file at {{spec_file_path}} to JSON, DO NOT implement the spec.
Create the worklist.json in the current working directory which will contain the requirements present in the spec.
Make sure you adhere to the following JSON format:

{
  "spec_file_path": "{{spec_file_path}}",
  "items": [
    { "id": "REQ-001", "completed": false, "notes": "" },
    { "id": "REQ-002", "completed": false, "notes": "" }
  ]
}

Important: You must ONLY create the worklist.json file do not implement anything.
"#;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { file, provider } => {
            handle_init(file, provider).context("Error during init")?;
        }
        Commands::Run { provider } => {
            handle_run(provider).context("Error during run")?;
        }
    }

    Ok(())
}

fn handle_init(file: &str, provider: &Provider) -> Result<()> {
    if !Path::new(file).exists() {
        bail!("Spec file not found at: {}", file);
    }

    let prompt = INIT_PROMPT_TEMPLATE.replace("{{spec_file_path}}", file);

    println!("Initializing worklist using provider: {:?}...", provider);
    execute_agent(&prompt, provider).context("Failed to execute agent for init")?;

    if !Path::new("worklist.json").exists() {
        bail!("Agent failed to create worklist.json");
    }

    println!("Successfully created worklist.json");

    Ok(())
}

fn handle_run(provider: &Provider) -> Result<()> {
    let worklist_path = "worklist.json";
    if !Path::new(worklist_path).exists() {
        bail!("worklist.json not found. Run 'bob init' first.");
    }

    loop {
        let worklist_content = fs::read_to_string(worklist_path)
            .context(format!("Failed to read {}", worklist_path))?;

        let worklist: Worklist = serde_json::from_str(&worklist_content)
            .context(format!("Failed to parse {}", worklist_path))?;

        let all_completed = worklist.items.iter().all(|item| item.completed);
        if all_completed {
            println!("All requirements completed!");
            break;
        }

        let prompt = RUN_PROMPT_TEMPLATE.replace("{{spec_file_path}}", &worklist.spec_file_path);

        println!("Running agent loop with provider: {:?}...", provider);
        let output = execute_agent(&prompt, provider).context("Failed to execute agent loop")?;

        if !output.contains("BOB_COMPLETED") {
            bail!("Agent finished without outputting BOB_COMPLETED. Stopping loop.");
        }
    }

    Ok(())
}

fn execute_agent(prompt: &str, provider: &Provider) -> Result<String> {
    let mut temp_file = NamedTempFile::new().context("Failed to create temporary file")?;
    write!(temp_file, "{}", prompt).context("Failed to write prompt to temporary file")?;
    let temp_path = temp_file
        .path()
        .to_str()
        .ok_or(anyhow!("Invalid temp path"))?;

    let command_str = match provider {
        Provider::Gemini => format!("cat {} | gemini -y", temp_path),
        Provider::Claude => format!(
            "claude --dangerously-skip-permissions --print < {}",
            temp_path
        ),
    };

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(&command_str)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context(format!("Failed to spawn command: {}", command_str))?;

    let mut stdout_captured = String::new();
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line.context("Failed to read line from agent stdout")?;
            println!("{}", line);
            stdout_captured.push_str(&line);
            stdout_captured.push('\n');
        }
    }

    let status = child
        .wait()
        .context("Failed to wait for agent process to finish")?;

    if !status.success() {
        bail!("Agent execution failed with status: {}", status);
    }

    Ok(stdout_captured)
}
