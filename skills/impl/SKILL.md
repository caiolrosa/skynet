---
name: impl
description: "Implement code changes with language-specific context. Use when the user provides lang:<languages> to load best-practice guidelines for each language before implementing. Triggers on: /impl lang:js,css fix the header layout"
user-invocable: true
argument-hint: "lang:<lang1,lang2,...> <implementation request>"
---

# Implementation with Language Context

Implement code changes informed by language-specific best practices and conventions. Each language context file provides guidelines the agent MUST follow when writing or modifying code in that language.

---

## Core Principles

1. **NEVER GUESS.** If anything is unclear, ambiguous, or has multiple valid interpretations — ASK. Do not assume, infer, or fill in gaps with your own judgment.
2. **NEVER pick a default without confirmation.** If there are multiple valid approaches, libraries, patterns, or structures — present the options to the user and wait for their choice.
3. **ASK before implementing.** When requirements are incomplete, vague, or could go multiple ways, stop and ask clarifying questions using `AskUserQuestion`. Do NOT silently resolve ambiguity.
4. **Surface error scenarios and edge cases.** Actively look for things that could go wrong — empty inputs, null values, network failures, race conditions, boundary values, invalid states. When you find them, present them to the user and ask how they should be handled. Do NOT silently decide on error handling behavior.
5. **Project conventions first.** If the codebase already has established patterns, follow them — but if you're unsure whether a pattern is intentional, ask.

---

## How It Works

1. Parse the `lang:` parameter from `$ARGUMENTS` to extract the comma-separated list of languages
2. Load the corresponding language context file for each language
3. Ask clarifying questions about anything unclear in the user's request
4. Identify error scenarios and edge cases, ask the user how to handle them
5. Use the loaded context as guidelines when implementing the user's request

---

## Argument Parsing

`$ARGUMENTS` follows the format: `lang:<lang1,lang2,...> <the user's implementation request>`

**Parse rules:**

- Extract the token that starts with `lang:` — everything after `lang:` up to the next whitespace is the comma-separated language list
- Split the language list by commas. Trim whitespace from each language key and lowercase it
- Everything in `$ARGUMENTS` after the `lang:...` token is the **implementation request**
- If `$ARGUMENTS` is empty or has no `lang:` token, ask the user which languages to load and what to implement

**Examples:**

| Input | Languages | Request |
|---|---|---|
| `lang:js,css fix the header` | js, css | fix the header |
| `lang:go refactor the auth module` | go | refactor the auth module |
| `lang:ts,css build a login form` | ts, css | build a login form |

---

## Language Context Loading

For each parsed language key, load the matching context file from the `langs/` directory:

- `js` → [langs/ts.md](langs/ts.md)
- `ts` → [langs/ts.md](langs/ts.md)
- `css` → [langs/css.md](langs/css.md)
- `ruby` → [langs/ruby.md](langs/ruby.md)
- `rust` → [langs/rust.md](langs/rust.md)
- `go` → [langs/go.md](langs/go.md)
- `sh` → [langs/sh.md](langs/sh.md)

If a language key has no matching file, inform the user that no context file exists for that language and ask if they want to proceed without it or provide their own guidelines.

---

## Implementation Flow

### Step 1: Load Context

Read each language context file identified above. These files contain guidelines, conventions, and best practices that MUST be followed when writing code in that language.

### Step 2: Clarify the Request

Before writing any code, analyze the user's implementation request and identify gaps or ambiguities. Use `AskUserQuestion` to resolve them. Common things to ask about:

- **Scope:** What exactly should be changed? Which files, components, or modules are in scope?
- **Behavior:** What should happen in the happy path? What about error cases?
- **Approach:** If there are multiple valid ways to implement the request, present them as options and let the user choose
- **Dependencies:** Should you use an existing library, or write it from scratch? Which version?
- **Naming:** If new functions, classes, variables, or files need to be created, confirm naming with the user unless existing patterns make the choice obvious

Do NOT proceed to implementation until you have enough clarity to write code confidently. If the request is already clear and specific, you may proceed — but err on the side of asking.

### Step 3: Explore the Codebase

Before writing any code, explore the relevant parts of the codebase to understand:

- Existing patterns and conventions already in use
- Files that will be modified or created
- Related code that the changes interact with
- Project-specific configuration (linters, formatters, build tools)

If you discover something unexpected (e.g., an existing implementation that partially covers the request, conflicting patterns, or architectural decisions that affect the approach), surface it to the user and ask how to proceed.

### Step 4: Identify Error Scenarios and Edge Cases

Before implementing, think through what could go wrong. Actively look for:

- **Invalid or missing inputs** — null, undefined, empty strings, empty arrays, missing fields
- **Boundary values** — zero, negative numbers, maximum lengths, overflow, off-by-one
- **External failures** — network errors, API timeouts, unavailable services, malformed responses
- **Concurrency issues** — race conditions, duplicate submissions, stale state
- **User behavior** — double clicks, navigation mid-operation, unexpected input formats
- **State problems** — invalid transitions, corrupted data, partial failures
- **Security concerns** — injection, unauthorized access, data leakage

Present the identified error scenarios and edge cases to the user using `AskUserQuestion`. For each one, describe the scenario and ask how it should be handled. Provide sensible options when possible, but do NOT pre-select or implement a default behavior without the user's explicit approval.

If no meaningful edge cases or error scenarios exist for the request, skip this step.

### Step 5: Implement

Execute the user's implementation request, following:

1. **Project conventions first** — if the codebase already has established patterns, follow them even if they differ from the language context file. Always follow project standards and patterns first.
2. **Language context guidelines** — apply the loaded guidelines for each language being used
3. **Minimal changes** — only change what is necessary to fulfill the request. Do not refactor surrounding code, add unnecessary abstractions, or "improve" unrelated code
4. **No hasty abstractions** — do not introduce abstractions prematurely. Duplication is far cheaper than the wrong abstraction. Only abstract when a clear, repeated pattern has emerged and the abstraction is justified by current needs, not hypothetical future ones.
5. **No premature optimization** — write clear, correct code first. Do not optimize for performance unless there is a measured bottleneck or the user explicitly requests it. Readable code that works is always better than clever code that is fast for no reason.
6. **Ask when uncertain** — if during implementation you encounter a decision point not covered by the user's instructions or the language context, stop and ask. Do NOT make a judgment call silently.

### Step 6: Verify

After implementing, verify the changes:

- Confirm all modified files are syntactically valid
- Run existing tests if applicable
- Check that the implementation aligns with both the language guidelines and the user's request
- If any tests fail or issues arise, inform the user and ask how they want to handle it rather than silently fixing things
