# Skills

Reusable skills for AI coding agents. Each subdirectory is a self-contained skill with a `SKILL.md` definition.

## Available skills

- **[probe](probe/)** — Interview you about a plan or decision until every branch is settled.

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
