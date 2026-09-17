# Issues skill

Turns agreed work into one markdown file per unit of it, each standalone enough that an agent implements it without reading the spec.

Last in the chain: [probe](../probe/) settles the decisions, [spec](../spec/) writes them up, `issues` cuts the work into files an agent picks up one at a time. No issue tracker, no API calls — the files are the deliverable.

## Files

- **SKILL.md** — Skill definition.
- **example/** — A filled-in set, sliced from [spec/example.md](../spec/example.md). The target the skill writes toward.

## Installation

```sh
cp -r skills/issues ~/.claude/skills/issues
```

## How it works

`/issues path/to/spec.md` reads that document. With no path it uses the spec or the discussion from the session. With nothing at all it asks one round of up to four questions, then writes.

It explores the repo for conventions, cuts the work by area, and writes `issues/01_snake_case_title.md` beside the source document, zero-padded in dependency order. No index file: the filenames are the queue.

Each file is a briefing:

```
# 03 — Retry worker, backoff and dead state

Depends on: 01

## Context
## Build          — a line of intent, then the artifact
## Constraints    — decisions and repo conventions, one list
## Done when
## Tests
## Out of scope
```

It closes with the coverage map and nothing else:

```
01_delivery_tables     ← spec 1
02_publish_writes_row  ← spec 2
03_retry_worker        ← spec 3, 4
```

It stops rather than guessing on: a dependency cycle, a repo that contradicts a decision, and an `issues/` directory that already exists.

## Design decisions

- **Each file is standalone.** An agent implementing issue 03 never opens the spec. Pointer-style issues never drift, but they defeat the point — the agent ends up reading the whole document anyway.
- **Artifacts, not file paths.** Issues show the migration, the payload, the signature, in the repo's idiom. The names inside an artifact are the artifact. A path is a pointer that goes stale between filing and implementing, and a stale one sends an agent confidently to the wrong place.
- **More concrete than the spec, never more decided.** Detail the spec left to the implementer belongs here. A decision the spec didn't make belongs nowhere.
- **One area, one issue, always.** Migration, request path, worker, frontend view — each on its own. A shared code dependency becomes a `Depends on` line plus the contract written out in full inside the dependent issue. An "unless they share code" exception sounds reasonable and turns every slice into a judgment call.
- **Entries in the same area merge only when neither half can be verified alone.** A worker's backoff schedule and the state it backs off into are one issue; two endpoints are two.
- **Decisions and conventions are one list.** Both constrain the agent identically, and where a rule came from changes nothing about honoring it.
- **No ceiling on `Done when`, and no cap on issue count.** The slicing rules decide the cuts; length never does.
- **Tests name the case and the boundary that makes it bite** — "a sixth failure moves it to dead: attempt_count 5, endpoint returns 500". Not test code: `impl` writes one test at a time, and a bulk suite handed to it describes behavior nobody built yet.
- **At most one diagram per issue, lifecycle only.** Issues get read in terminals, where mermaid is raw source, so the same facts stay readable as text.
- **No traceability line in the file.** Re-slicing breaks any 1:1 mapping to the source document, and a pointer into a document that got re-cut is a pointer that lies. The coverage map in chat gives it back at the one moment someone can act on it.
- **The QA Plan becomes the last issue**, depending on all others. It's the only place the assembled work gets checked, because no single issue proves it.
- **Open questions are not carried.** Issues are written as though they were settled, and nothing warns the implementing agent. Deliberate: a warning in a file an agent reads alone is a warning an agent ignores.
- **A repo that contradicts a decision stops the run.** Same rule `spec` uses. Since issues carry no warnings, a contradicted decision would reach the agent as fact.
- **An existing `issues/` directory stops the run, and there is no append mode.** Regenerating destroys hand edits and in-flight work; renumbering repoints issues an agent is already building.
- **A worked example ships with the skill.** Rules describe good slicing; an example shows it. This is the main defense against output that reads generated.
- **It never implements.** Output is the files.
