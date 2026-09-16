# Impl skill

Builds the work in one issue file, one test at a time, then hands the diff to code review.

Fourth in the chain: [probe](../probe/) settles the decisions, [spec](../spec/) writes them up, [issues](../issues/) slices them into briefings, `impl` builds one. It ends by calling `cr`, and it never commits.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/impl ~/.claude/skills/impl
```

## How it works

`/impl issues/03-thing.md` reads that issue. Given a spec or a plain sentence instead, it says once what's missing and works from what's there.

Before writing, it reads the repo's standards docs, nearby code of the same kind, how the repo verifies, and `git status` — so anything already dirty stays out of its report.

Then the loop, one slice at a time: one failing test, the smallest code that passes it, a fast check, repeat. Per slice it runs the single test file and a typecheck; once at the end it runs the full suite and lint.

When the work is verified, it sends the issue path to `cr` in a sub-agent and waits. It applies the findings it agrees with and re-verifies. What it then reports is only what's left to decide: findings that still stand, in full. What it applied and what the diff already satisfied are counted in one line, never listed, and an axis that didn't run says so.

It stops and asks on four things: the repo contradicts a decision that binds the issue, five failed attempts at the same test, a final suite failure in code it didn't touch, or it can't work out how the repo verifies.

## Design decisions

- **One issue per run.** The issue is already standalone by design, so nothing else needs to be open. Running a queue means invoking it once per file.
- **It asks only when blocked.** `probe`, `spec` and `issues` exist to settle decisions upstream. An implement step that re-asks scope, approach and error handling makes the whole chain pointless.
- **No language context files.** The old version of this skill shipped six `langs/*.md` guides. The repo's own standards docs and the issue's `Conventions to follow` already carry it, and the guides drifted from what the repo actually did.
- **It does not clean up its own diff.** Writing the minimum and letting `cr` judge it is the point. Reviewing your own work in the context that produced it misses exactly what a separate review catches.
- **Code review runs in a sub-agent.** The review reads a lot; the report is short. Only the report comes back.
- **The review is reported by what's left to decide.** A finding that still stands — not applied, whatever the reason — comes back verbatim with the reason it stands. That's what the user reads. Findings it applied, and ones the diff already satisfied, are counted in the opening line and never listed — the diff already says what changed, and a roll-call of fixed things is noise nobody reads.
- **Every axis gets a heading and a line**, whether nothing stood on it or it never ran. Two headings where three were promised, or a skipped axis that looks like a clean one, is how a review that missed a whole check passes for a pass.
- **`cr` is never asked for less.** It reports everything it finds, at whatever length that takes. The trimming happens once, here, on the way to the user. A reviewer that holds findings back to keep its report short is deciding what the caller gets to weigh.
- **The issue beats the review.** A finding that contradicts `Decisions that bind this` is dismissed with the decision quoted. `cr` reads the issue too, but a settled decision outranks a reviewer's read of it.
- **It commits nothing and stages nothing** — not even `git add -N`. The tree is left dirty and the report names the files, so the user reviews before anything lands.
- **The issue file is read-only.** No ticked boxes, no appended log. Nothing on disk records that an issue was implemented, so a second run redoes it.
- **Integration over unit.** Tests go through the real interface with the real code behind it. Unit tests are reserved for logic worth stressing on its own — complex parsing, non-trivial calculation, boundaries a caller can't reach from outside. Repo conventions come first where they exist. Same rule [issues](../issues/) uses when it names test cases.
- **A test that passes the moment it's written is treated as broken.** No red means it proves nothing.
- **Work with nothing runnable skips the test** — markdown, config, CI. It says so in one line rather than faking an assertion. The escape doesn't apply when a runner exists and the change touches code it executes.
