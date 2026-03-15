use crate::agent_manager::Provider;
use anyhow::{Context, Result, anyhow, bail};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub trait AgentCommandRunner {
    fn execute(&self, prompt: &str, provider: &Provider) -> Result<String>;
}

pub struct AgentRunner;

impl AgentCommandRunner for AgentRunner {
    fn execute(&self, prompt: &str, provider: &Provider) -> Result<String> {
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
}
