# Bob (the builder)

`bob` is a CLI tool that orchestrates AI agents to automate software development tasks. It uses a Markdown specification to generate and manage a `worklist.json` file, executing tasks in a loop until all requirements are completed.

## Features

- **Init**: Bootstrap a project by converting a Markdown spec into a `worklist.json`.
- **Run**: Execute an agent-driven development loop using Gemini or Claude.
- **State Management**: Tracks progress and notes for each requirement in `worklist.json`.
- **Provider Support**: Integrated with `gemini` and `claude` CLI tools.

## Usage

### Initialize bob
Create a `worklist.json` from a specification file:
```bash
bob init -f specs/your-spec.md --provider gemini
```

### Run the development loop
Execute the requirements defined in the worklist:
```bash
bob run --provider gemini
```

## How it works

1. **Init**: Bob sends a prompt to the chosen AI provider to parse your spec and create `worklist.json`.
2. **Run**: Bob enters a loop, sending the current state and spec path to the agent.
3. **Execution**: The agent implements one requirement at a time, updates `worklist.json`, and outputs `BOB_COMPLETED`.
4. **Completion**: Bob continues the loop until all items in the worklist are marked as completed.

## Worklist Structure

The `worklist.json` file tracks the project's progress:

```json
{
  "spec_file_path": "specs/spec-bob-cli.md",
  "items": [
    { "id": "REQ-001", "completed": true, "notes": "Implemented worklist structure" },
    { "id": "REQ-002", "completed": false, "notes": "" }
  ]
}
```
