# Skills

Reusable skills for AI coding agents. Each subdirectory is a self-contained skill with a `SKILL.md` definition.

## Available skills

- **[probe](probe/)** — Interview you about a plan or decision until every branch is settled.
- **[spec](spec/)** — Turn those settled decisions into one document a team can agree on, sliced into work items ready to become issues.

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
