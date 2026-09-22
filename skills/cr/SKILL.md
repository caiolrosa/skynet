---
name: cr
description: "Code reviews changes in the current branch or commit"
user-invocable: true
argument-hint: "[path to plan file] [which changes, current branch or starting commit]"
---

# Cr
Your job is to review code not fix it or change it in any way.

---

## Core rules
- You review based on the changes in the current branch regardless if they are staged or not. If not in a branch ask for a commit to start your review from.
- Use git rev-parse to make sure you don't miss unstaged changes.
- Read the documentation, context or description provided so you know what those changes are about.
- Only review and report findings, never edit, stage or commit changes.
- Every finding is one or two lines, file, issue description, suggested fix.
- Don't run the tests, that's the user's or whoever called the skill job.
- Follow the standards described below during review, if the project deviates from these standards raise it, but don't mark as an issue, just a recommendation.
- Exclude noise, lockfiles, generated code, vendored dependencies, snapshots etc, don't belong in the review. You may mention these files as part of the changes.
- Look for repository standards, `CLAUDE.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CODING_STANDARDS.md`, etc.
- Use OWASP top 10.

---

## Review rules
- Review against the repository's standards first, if none available the standards defined below.
- Review against the coding standards below.
- Review against the test standards below.
- Review against the correctness standards below.
- Review against the security standards below.
- Review against the plan, the implementation and plan must agree.

---

## Coding Standards
- Avoid Hasty Abstractions: an abstraction built before the duplication showed its shape. One helper now serves two callers that only look alike is worse than the duplication itself.
- Composition over inheritance: inheritance usually leads to more coupling and hiding complexity in deep layers that become unwieldy fast, composition keeps the code simpler and modular.
- Tightly dependent code is likely a smell for a missing interface.
- Names should declare what they hold or their intent, if none is achieved the name is bad or the design is murky.
- Shotgun Surgery: one change forced edits in many modules or files, it's likely breaking single responsibility somewhere in the chain.
- The middle man problem: a function that only passes the call along can be cut to call the thing directly.
- N+1 queries are usually a performance low hanging fruit.
- Errors should always be handled even if it's just returning a generic error message back to the user
- Security is very important use OWASP top 10 to validate the changes.

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

## Security Standards
- Broken Access Control: look for any missing policies or authorizations in the changes that would lead to unauthorized access.
- Security Misconfiguration: look for any configuration in the changes that is setup insecurely or could be hardened.
- Software Supply Chain Failures: look for any added dependencies in the changes that have vulnerabilities unresolved for the version added.
- Cryptographic Failures: look for any sensitive data in the changes that should be encrypted during transport.
- Injection Failures: look for any untrusted input in the changes that is not being sanitized before sent to any interpreter, databases, browser, command line, etc.
- Authentication Failures: look for flows that should be placed behind authentication in the changes and aren't.
- Dangerous Logging: look for any sensitive data in the changes that are being logged and shouldn't be.

---

## Correctness Standards
- Look for bugs in the implementation, details the implementer might have missed that will cause issues.
- Look for edge cases the implementer might not have considered.
- Look for concurrency issues the tests might not catch.
