use crate::agent_runner::AgentCommandRunner;
use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, ValueEnum, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
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

pub struct AgentManager<R: AgentCommandRunner> {
    runner: R,
}

impl<R: AgentCommandRunner> AgentManager<R> {
    pub fn new(runner: R) -> Self {
        Self { runner }
    }

    pub fn init(&self, file: &str, provider: &Provider) -> Result<()> {
        if !Path::new(file).exists() {
            bail!("Spec file not found at: {}", file);
        }

        let prompt = INIT_PROMPT_TEMPLATE.replace("{{spec_file_path}}", file);

        println!("Initializing worklist using provider: {:?}...", provider);
        self.runner
            .execute(&prompt, provider)
            .context("Failed to execute agent for init")?;

        if !Path::new("worklist.json").exists() {
            bail!("Agent failed to create worklist.json");
        }

        println!("Successfully created worklist.json");

        Ok(())
    }

    pub fn run(&self, provider: &Provider) -> Result<()> {
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

            let prompt =
                RUN_PROMPT_TEMPLATE.replace("{{spec_file_path}}", &worklist.spec_file_path);

            println!("Running agent loop with provider: {:?}...", provider);
            let output = self
                .runner
                .execute(&prompt, provider)
                .context("Failed to execute agent loop")?;

            if !output.contains("BOB_COMPLETED") {
                bail!("Agent finished without outputting BOB_COMPLETED. Stopping loop.");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRunner;

    impl AgentCommandRunner for MockRunner {
        fn execute(&self, _prompt: &str, _provider: &Provider) -> Result<String> {
            Ok(String::new())
        }
    }

    #[test]
    fn test_init_missing_spec() {
        let manager = AgentManager::new(MockRunner);
        let result = manager.init("non_existent_spec.md", &Provider::Claude);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Spec file not found at: non_existent_spec.md"
        );
    }

    #[test]
    fn test_run_missing_worklist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let manager = AgentManager::new(MockRunner);
        let result = manager.run(&Provider::Gemini);

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "worklist.json not found. Run 'bob init' first."
        );
    }
}
