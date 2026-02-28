# Impl skill

A Claude Code skill that loads language-specific best practices before implementing code changes.

## Files

- **SKILL.md** — Skill definition. Describes argument parsing, context loading, and the implementation flow.
- **langs/** — Language context files with guidelines for each supported language:
  - `ts.md` — TypeScript/JavaScript
  - `css.md` — CSS
  - `go.md` — Go
  - `rust.md` — Rust
  - `ruby.md` — Ruby
  - `sh.md` — Shell

  Add or remove language files in `langs/` to customize which languages are available.

## Installation

Copy the skill files to the appropriate skills directory:

```sh
cp -r skills/impl ~/.claude/skills/impl  # Claude Code
cp -r skills/impl ~/.gemini/skills/impl  # Gemini CLI
```

## Usage

```
/impl lang:<languages> <request>
```

Examples:

```
/impl lang:ts,css build a login form
/impl lang:go refactor the auth module
/impl lang:rust add error handling to the parser
```

## How it works

1. **Parse** — Extract languages from `lang:` and the implementation request.
2. **Load context** — Read the matching `langs/*.md` files for each language.
3. **Clarify** — Ask about scope, behavior, and approach if anything is ambiguous.
4. **Edge cases** — Identify error scenarios and ask the user how to handle them.
5. **Implement** — Write code following project conventions first, then language guidelines.
6. **Verify** — Check syntax, run tests, confirm alignment with the request.
