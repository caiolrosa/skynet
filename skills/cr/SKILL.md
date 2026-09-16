---
name: cr
description: "Review a diff along three axes — standards, spec and correctness — in parallel sub-agents, and report the findings without touching the code. Use after impl, or on a branch or working tree that needs a review. Triggers on: review this, code review, cr this, review the diff, review my changes, review since main."
user-invocable: true
argument-hint: "[path to issue file] [fixed point]"
---

# Cr

Review a diff and report what's wrong with it. Three axes, three sub-agents, one report.

You review. You do not fix what you find and you do not touch the tree. See [Never fix](#never-fix).

---

## Core rules

1. **Report only.** No edits, no staging, no commits, no files written.
2. **The three axes stay separate.** Don't merge their findings and don't rank across them.
3. **Every finding is one or two lines** — file, claim, suggested fix, marking. Nothing longer.
4. **Say what didn't run.** An axis that skipped or failed is named with the reason. Never dropped.
5. **Don't run the test suite.** Whoever called you already did.

---

## Input

| Given | Use |
|---|---|
| `/cr issues/03-thing.md` | That issue as the spec source, the uncommitted tree as the diff. |
| `/cr` | The uncommitted tree. An issue from this session, if there is one. |
| `/cr <branch_name>`, `/cr <commit_sha>`, `/cr <tag>`, `/cr HEAD~5` | `git diff <point>` — everything committed since that point, plus the uncommitted tree. |
| `/cr issues/03-thing.md <branch_name>` | Both. |
| `/cr`, clean tree, no fixed point | Stop. Nothing to review. |

Order doesn't matter. A path that exists is the issue; anything else is a fixed point.

---

## Before you spawn

Four things, in this order. All of them are cheap, and all of them fail better here than inside three parallel sub-agents.

### 1. Pin the diff

| Fixed point given | Command |
|---|---|
| No | `git diff HEAD` |
| Yes | `git rev-parse <point>` to confirm it resolves, then `git diff <point>` |

Both forms compare against the working tree, so uncommitted work is always in scope. That's deliberate: `impl` never commits, so a review that only reads commits reviews nothing.

**`git diff` misses new files.** They're untracked, and `impl` doesn't stage. Run `git status --porcelain` and read every `??` file yourself — those are part of the change.

With a fixed point, also capture `git log <point>..HEAD --oneline` and pass it along.

### 2. Exclude the noise

Drop lockfiles, generated code, vendored dependencies and snapshot files from what you send. Name them in one line in the report so nobody thinks they were reviewed.

### 3. Find the spec source

The issue file, given as an argument or open in this session. Nothing else counts.

No issue file means the Spec axis doesn't run. Don't substitute a branch name, a commit message or a plain sentence — a yardstick you invented produces findings against requirements nobody agreed to.

### 4. Find the standards sources

Whatever the repo documents about how code is written: `CLAUDE.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CODING_STANDARDS.md`. List the paths; the sub-agent reads them.

If the diff touches prose or markdown, `readability.md` is a standards source too.

---

## The three axes

Spawn all three in **one message** so they run in parallel. They don't share context, and that's the point: an axis that has seen another axis's findings starts agreeing with them.

Every brief carries the diff commands, the commit list, the exclusion list, and the path to the issue file. All three agents read the issue. An agent that hasn't read it flags work the issue mandated.

### Standards

Reviews against, in this order:

1. **The repo's own documented standards.** Cite the file and the rule.
2. **The smell baseline** below, pasted into the brief in full. The agent has no other access to it.
3. **`readability.md`**, where the diff touches prose or markdown.
4. **The test rules** below.

Two rules bind it: **the repo overrides** — where a documented standard endorses something the baseline would flag, suppress the smell — and **the baseline is always a judgement call**, never a hard violation. Skip anything tooling already enforces.

Brief:

> Report, per file and hunk: (a) every place the diff breaks a documented standard, citing the file and the rule; (b) any baseline smell, named, with the hunk quoted; (c) any test rule broken. Mark each finding **hard** or **judgement**. A documented standard can be hard; a baseline smell never is. Skip what tooling enforces. One or two lines per finding, however many you find.

#### The smell baseline

Each entry reads *what it is* → *how to fix*. Match them against the diff.

- **Avoid Hasty Abstractions**: an abstraction built before the duplication showed its shape. One helper now serves two callers that only look alike. → inline it back. Duplicate until the real shape shows.
- **Composition over inheritance**: a subclass exists to reuse code, not to stand in for its base. → hold the thing instead of extending it.
- **Mysterious Name**: the name doesn't say what it does or holds. → rename it. If no honest name comes, the design is murky.
- **Data Clumps**: the same few fields travel together everywhere. → bundle them into one type, pass that.
- **Primitive Obsession**: a string or a number stands in for a real concept. → give the concept its own type.
- **Shotgun Surgery**: one change forced edits in many files. → gather what changes together into one place.
- **Divergent Change**: one file is edited for unrelated reasons. → split it, so each part changes for one reason.
- **Message Chains**: the caller walks `a.b().c().d()` to get what it needs. → hide the walk behind one method on `a`.
- **Middle Man**: a function that only passes the call along. → cut it, call the real thing.

Duplication on its own is not a finding. Never ask for an extraction — the wrong abstraction costs more than the copy. Flag the reverse: an abstraction that arrived before its shape was clear.

#### The test rules

Tests are part of the diff and nothing else reviews them. The repo's own testing conventions come first. Where it has none:

- **Goes through the public interface.** A test that fakes out the project's own code, reaches into private functions, or checks the result through a side channel — reading the database directly instead of calling the function that reads it — is tied to the implementation.
- **Mocks only at system boundaries** — external APIs, databases and the file system where a real one isn't practical, time, randomness. Never the project's own modules.
- **Takes its expected value from somewhere independent** — a known-good literal, a worked example, the issue. Not a value the test computes the way the code does, because both copies share the same mistake.
- **Names behaviour, not mechanics.** *"user can check out with a valid cart"*, not *"checkout calls processPayment"*.
- **One behaviour per test**, with as many assertions as that behaviour needs.
- **Would have failed before the change.** A test asserting only what the code already did proves nothing. Flag it; don't run it.

### Spec

Brief:

> Read the issue file. Report: (a) requirements it asked for that are missing or only partly done; (b) behaviour in the diff nobody asked for; (c) requirements that look implemented but implemented wrong. Quote the issue line for every finding. Work from `Done when`, `Tests` and `Out of scope`. Mark each finding **hard** or **judgement**. One or two lines per finding, however many you find.

**No issue file, no Spec axis.** Skip it and say so in the report.

### Correctness

The bug hunt: unhandled edges, wrong logic, races, boundaries the tests miss.

Brief:

> Hunt for bugs in the diff — unhandled edges, wrong logic, boundary and error paths, concurrency, anything the tests would miss. Label every finding **certain** (it is wrong, and here's the input that breaks it), **likely** (wrong under conditions the code doesn't rule out), or **speculative** (worth a look, unproven). Say why the existing tests don't catch it. One or two lines per finding, however many you find.

**Nothing runnable in the diff, no Correctness axis.** A change that's all markdown, config or docs has no behaviour to be wrong. Skip it and say so.

---

## The report

Three headings, each verbatim from its sub-agent or lightly cleaned. Don't merge findings, don't rerank, don't pick an overall worst. The separation is the product — a Standards pass can mask a Spec failure and the other way round, which is why they never share a list.

```markdown
Reviewed `git diff main` — 14 files, 3 untracked.
Excluded: pnpm-lock.yaml, src/generated/.

## Standards
- `src/fees/refund.ts:42` — **hard** — CONTRIBUTING.md says handlers return `Result`, this throws. Fix: return `Result`.
- `src/fees/refund.ts:88` — **judgement** — `formatAmount` takes a `mode` flag to serve two callers that only look alike. Fix: inline it back.

## Spec
- Missing — issue says "Done when: partial refunds are rejected with a 422". No 422 path. Fix: add it.

## Correctness
- `src/fees/refund.ts:51` — **likely** — `amount` of 0 divides by `total` before the guard. Fix: move the guard up.

Standards 2 · Spec 1 · Correctness 1
```

No cap on how many findings come back. The bound is per finding, not per report.

**An axis that skipped or failed is named**, with the reason, in place of its findings:

```markdown
## Spec
Did not run — no issue file.
```

Two sections silently appearing where three were promised is how a change gets trusted for a check that never happened.

---

## Stopping

Three things stop the run before any sub-agent spawns:

| Stop | Why |
|---|---|
| The fixed point doesn't resolve | `git rev-parse` failed. Say which ref. |
| The diff is empty | Nothing to review. Say so, and mention a fixed point if the tree is clean. |
| Not a git repo | There's no diff to take. |

Everything else runs.

---

## Never fix

You produce findings. You do not apply them, you do not "just fix the obvious one", and you do not touch the working tree — not even to format a file you're reporting on.

A reviewer that edits the code it reviewed is the thing this skill exists to replace. The caller decides what to apply, and when `impl` is the caller, the issue beats the review.
