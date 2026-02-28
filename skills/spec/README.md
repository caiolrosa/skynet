# Spec skill

A Claude Code skill that generates technical specification documents optimized for AI implementation.

## Files

- **SKILL.md** — Skill definition. Describes the 7-phase wizard that guides spec creation.
- **template.md** — Skeleton used to structure the generated spec. Only relevant sections are included.
- **example.md** — Reference output showing the expected format, tone, and level of detail.

## Installation

Copy the skill files to the appropriate skills directory:

```sh
cp -r skills/spec ~/.claude/skills/spec  # Claude Code
cp -r skills/spec ~/.gemini/skills/spec  # Gemini CLI
```

## How it works

The skill runs a 7-phase flow:

1. **Gather idea** — Get the feature description from the user.
2. **Scan codebase** — Understand existing patterns, conventions, and constraints.
3. **Clarify** — Ask targeted questions (up to 3 rounds) covering scope, behavior, data, security, and dependencies.
4. **Edge cases** — Systematically identify boundary conditions and failure modes.
5. **Output path** — Choose where to save the spec file.
6. **Generate** — Write the spec using `template.md` as the structure and `example.md` as the tone reference.
7. **Validate** — Check for weasel words, broken dependencies, stub sections, and size.

## Output format

Specs use numbered requirements (`REQ-001`, `REQ-002`, ...), each containing:

- What / Where / Behavior
- Edge cases and validation rules
- Code examples and dependencies

Other sections (architecture, data model, error handling, testing strategy, implementation plan) are included only when relevant.

## Design goals

- No ambiguity — concrete values, explicit behavior, precise language.
- Self-contained — an agent can implement the spec without follow-up questions.
- Verifiable — every requirement has clear completion criteria.
