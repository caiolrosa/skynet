# Skills

Reusable skills for AI coding agents. Each subdirectory is a self-contained skill with a `SKILL.md` definition.

## Available skills

- **[probe](probe/)** — Interview you about a plan or decision until every branch is settled.
- **[spec](spec/)** — Turn those settled decisions into a document a team validates before the work starts: the end-state design, then the ordered changes.
- **[issues](issues/)** — Turn that document into one markdown file per unit of work, each standalone enough for an agent to implement on its own.
- **[impl](impl/)** — Build the work in one issue file, one test at a time, then hand it to code review.
- **[cr](cr/)** — Review the diff along three axes in parallel sub-agents, and report the findings without touching the code.

## Installation

Copy the skills you want to the appropriate directory:

```sh
cp -r skills/<skill> ~/.claude/skills/<skill>  # Claude Code
```

## Structure

Each skill follows the same layout:

```
<skill>/
├── SKILL.md        # Skill definition (required)
└── ...             # Supporting files (templates, examples, language contexts)
```

See each skill's README for details.
