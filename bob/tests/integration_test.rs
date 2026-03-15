use anyhow::Result;
use bob::agent_manager::{AgentManager, Provider};
use bob::agent_runner::AgentCommandRunner;
use std::cell::RefCell;
use std::fs;

struct MockRunner {
    call_count: RefCell<usize>,
}

impl MockRunner {
    fn new() -> Self {
        Self {
            call_count: RefCell::new(0),
        }
    }
}

impl AgentCommandRunner for MockRunner {
    fn execute(&self, prompt: &str, _provider: &Provider) -> Result<String> {
        let mut count = self.call_count.borrow_mut();
        *count += 1;

        // Bit dirty, but will do for now
        if prompt.contains("ONLY to convert the spec md file") {
            let worklist = r#"{
              "spec_file_path": "spec.md",
              "items": [
                { "id": "REQ-001", "completed": false, "notes": "" }
              ]
            }"#;
            fs::write("worklist.json", worklist).unwrap();
            Ok("".to_string())
        } else if prompt.contains("You are an autonomous coding agent") {
            let worklist_str = fs::read_to_string("worklist.json").unwrap();
            let mut worklist: serde_json::Value = serde_json::from_str(&worklist_str).unwrap();
            worklist["items"][0]["completed"] = serde_json::Value::Bool(true);
            worklist["items"][0]["notes"] = serde_json::Value::String("Done".to_string());
            fs::write("worklist.json", serde_json::to_string(&worklist).unwrap()).unwrap();

            Ok("BOB_COMPLETED\n".to_string())
        } else {
            Ok("".to_string())
        }
    }
}

#[test]
fn test_end_to_end_agent_loop() {
    let temp_dir = tempfile::tempdir().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    fs::write("spec.md", "# Dummy Spec\n- Requirement 1\n").unwrap();

    let runner = MockRunner::new();
    let manager = AgentManager::new(runner);

    let init_result = manager.init("spec.md", &Provider::Gemini);
    assert!(init_result.is_ok(), "Init should succeed");
    assert!(
        fs::metadata("worklist.json").is_ok(),
        "worklist.json should be created"
    );

    let run_result = manager.run(&Provider::Claude);
    assert!(run_result.is_ok(), "Run should succeed and complete loop");

    let final_worklist_str = fs::read_to_string("worklist.json").unwrap();
    let final_worklist: serde_json::Value = serde_json::from_str(&final_worklist_str).unwrap();
    assert!(
        final_worklist["items"][0]["completed"].as_bool().unwrap(),
        "Requirement should be completed"
    );

    std::env::set_current_dir(original_dir).unwrap();
}
