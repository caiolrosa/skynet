# Skills

Reusable skills for AI coding agents. Each subdirectory is a self-contained skill with a `SKILL.md` definition.

## Available skills

- **[spec](spec/)** — Generate technical specification documents.
- **[impl](impl/)** — Load language-specific best practices before implementing code changes.

## Installation

Copy the skills you want to the appropriate directory:

```sh
cp -r skills/<skill> ~/.claude/skills/<skill>  # Claude Code
cp -r skills/<skill> ~/.gemini/skills/<skill>  # Gemini CLI
```

## Structure

Each skill follows the same layout:

```
<skill>/
├── SKILL.md        # Skill definition (required)
└── ...             # Supporting files (templates, examples, language contexts)
```

See each skill's README for details.
