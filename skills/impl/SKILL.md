---
name: impl
description: "Build the work in a description or plan"
user-invocable: true
argument-hint: "[description or the path to a plan]"
---

# Impl

## Core rules
- Build the work from the user's description or the plan that was passed in.
- Use TDD, write tests than write the required code iterate on it until the test is green.
- Never force the tests to go green, if you have tried a few times and it is still red, stop and ask the user.
- The repository's standards must always be followed even if not in accordance with the standards defined in the standards sections.
- Never commit and never stage, you may ask the user if he wants you to commit, but that decision is the user's decision not yours.
- If you find something in the codebase that contradicts the plan or initial description, ask the user.
- Never guess, if in doubt ask the user.

---

## General Standards

- Always refer to `CLAUDE.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CODING_STANDARDS.md` — whatever documents how code in the repository is written.
- Whatever the plan or description says to match tells you where to look; the code tells you how it's actually done — layout, naming, how tests are written and where they live.
- Look for Makefiles, package scripts and if necessary CI config to understand how lint, format and test works in the project, if you can't find anything or is unsure, ask the user.
- Run `git status`. If it's already dirty, keep the list — those files are not yours, you don't touch them, and you name them separately at the end.

---

## Coding Standards

- Avoid hasty abstractions, hasty abstractions are much worse than repeating code in 2 or 3 different places, only abstract if it's a clear win, always tell the user about the abstractions you created.
- Avoid premature optimization, clear and understandable code is more important than the most optimized code, if unsure about it write the code and than ask the user about it.
- Composition over inheritance, keep the code simple and modular, avoid long inheritance chains.
- Avoid "go to definition hell", code you have to drill through many layers usually is bad code and it's a smell for poor abstractions.
- Tightly dependent code can be a smell for a missing interface, always plan around interfaces to keep the code easily testable.

---

## Testing Standards

- Follow the repository's own testing conventions first — its framework, its layout, its style.
- Integration tests over unit tests, unless the repository's conventions says otherwise, test the real flows the code goes through as much as possible.
- Unit tests should focus on stress testing specific logic, complex parsing, non-trivial calculation, state machines and any other complex logic that needs specific assurances.
- Mock at system boundaries only: external APIs, time and randomness, databases, cache and the file system when running the real thing isn't practical — prefer real files in a temp directory where you can.
- Test through public interfaces, the code behind it can completely change and if the functionality is still the same the test should pass.
- Tests should carry the behavior description and not the implementation detail in their name.
- Assertions that belong together should be kept together, validating fields of a response or object is one test, not many. Unrelated behavior is what warrants a separate test.
- Only compute values for assertions based on fixed fixtures, use static values to drive assertions and when necessary compute from these static values.

--- 

## Wrapping up
- Make sure all the project's quality gates are green: tests, format, lint and any other verifications the project uses.
- Summarize to the user what you implemented, keep it short and direct.
- Point out relevant information: abstractions created, performance and architecture considerations you encountered while building
- Offer the user to commit the work or call the code review skill `/cr`
