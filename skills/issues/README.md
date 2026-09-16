# Issues skill

Turns an agreed spec into one markdown file per unit of work, each standalone enough that an agent implements it without reading the spec.

Last in the chain: [probe](../probe/) settles the decisions, [spec](../spec/) writes them up and slices the work, `issues` turns each slice into a file an agent can pick up on its own. No issue tracker, no API calls — the files are the deliverable.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/issues ~/.claude/skills/issues
```

## How it works

`/issues path/to/spec.md` reads that document. With no path it uses the spec written earlier in the session. With no spec at all it stops — it only takes specs.

It explores the repo for conventions, slices the spec's work items, and writes `issues/01-<slug>.md` beside `spec.md`, zero-padded in dependency order. No index file: the filenames are the queue.

Each file is a briefing:

```
# 03 — Fee strategy for refunds

Spec: W4 · Depends on: 02

## Context
## Task
## Decisions that bind this
## Conventions to follow
## Done when
## Tests
## Out of scope
```

Sections with nothing in them are omitted rather than padded.

It stops rather than guessing on: a dependency cycle, input that isn't a spec, an `issues/` directory that already exists, and a repo that contradicts a decision the spec made.

## Design decisions

- **Each file is standalone.** An agent implementing issue 03 never opens `spec.md`. Pointer-style issues never drift, but they defeat the point — the agent ends up reading the whole document anyway.
- **No file paths, no symbol names.** A pointer that moved between filing and implementing sends an agent confidently to the wrong place. Issues carry patterns and conventions instead, which survive a refactor.
- **No index file.** The agent acts on one issue at a time, so an index serves nobody and drifts from the files it lists. Zero-padded names already sort into execution order.
- **The spec is the floor on granularity.** Items split freely when they span two separately verifiable things, but merge only when merging doesn't raise risk — and the skill asks first. Unattended, it never merges. Work the team agreed on shouldn't quietly vanish into another issue.
- **A split becomes `03a`/`03b`.** Nothing after it renumbers, and both files stay traceable to the same work item.
- **Tests are recommended per issue.** `spec` explicitly declines this job. Repo conventions come first; absent one, integration over unit, with unit tests reserved for logic worth stressing on its own — complex parsing, non-trivial calculation.
- **The Manual QA Plan becomes the last issue**, depending on all others. It's the only place the assembled work gets checked, because no single issue proves it — which is also why it isn't split across the issues it relates to.
- **Open questions are not carried.** Issues are written as though they were settled, and nothing warns the implementing agent. Deliberate: a warning in a file an agent reads alone is a warning an agent ignores.
- **A repo that contradicts a decision stops the run.** Same rule `spec` uses. Since issues carry no warnings, a contradicted decision would reach the agent as fact.
- **An existing `issues/` directory stops the run.** Regenerating would destroy hand edits and in-flight work with no record that it happened.
- **It never implements.** Output is the files.
