---
name: issues
description: "Turn an agreed spec into one markdown file per unit of work, each standalone enough that an agent implements it without reading the spec. Use after spec, when the document is settled and the work needs breaking into files an agent can pick up one at a time. Triggers on: file the issues, break this into issues, turn the spec into issues, generate the issues, slice this into tickets."
user-invocable: true
argument-hint: "[path to spec document]"
---

# Issues

Turn a spec the team already agreed on into one file per unit of work. Each file is a briefing: an agent opens it, implements it, and never reads the spec.

You slice and brief. You do not interview and you do not build. See [Never implement](#never-implement).

---

## Core rules

1. **The spec is the source of truth.** Don't interview. The two exceptions are [a merge you're unsure about](#slicing) and [a repo that disagrees](#when-the-repo-disagrees).
2. **One issue, one agent, one sitting.** Never write a line that assumes the reader has anything else open.
3. **Never name a file path or a symbol.** Paths and names go stale between filing and implementing, and a stale one sends an agent confidently to the wrong place. Carry patterns instead.
4. **Don't invent.** Nothing enters an issue that the spec didn't settle.
5. **Write plain and short.** Active voice, no hedging, no preamble.

---

## Input

| Given | Use |
|---|---|
| `/issues path/to/spec.md` | That document. |
| `/issues`, a spec was written this session | That document. |
| `/issues`, no spec anywhere | Stop. Say a spec is needed and recommend `/spec`. |

Only a spec document. Not a probe document, not a PRD, not a conversation.

### When it isn't a spec

If the document has no recognisable work items, stop and say so. Write nothing. Best-effort extraction from an unknown format produces issues nobody agreed on.

---

## Explore before writing

Read the repo for how this kind of work is already done: the layering, the naming, the shape of similar things, the test style. This is what fills **Conventions to follow**.

Carry patterns, never locations. *"Strategies register themselves at module load; mirror the refund one"* survives a refactor. *"Edit `src/fees/strategy.ts`"* doesn't.

**Skip exploring entirely** when the spec has no code behind it — a process, a piece of writing, a decision about the team. Those specs carry no **Touches** lines. Omit **Conventions to follow** everywhere rather than inventing conventions to fill it.

### When the repo disagrees

Exploring can turn up something that contradicts a decision the spec made. That is the only time you stop and ask.

Say what you found and what it breaks. Wait. Don't file the issue anyway with a note — nothing in an issue file warns an agent off, so a contradicted decision reaches it as fact.

---

## Slicing

Start from the spec's work items. One issue per `W` item is the default.

**Split** a `W` item when it spans two things that can be done and verified separately — when half of it has its own observable outcomes and its own tests, it is two issues.

Split issues take letter suffixes: `03a`, `03b`. Nothing after them renumbers. Both files carry `Spec: W3`.

**Merge** two `W` items only when merging doesn't raise the risk of the work. Ask first, through `AskUserQuestion`, naming both items and why they read as one edit. **When you can't ask, don't merge** — fall back to one-to-one and say so in the closing line.

**Never drop a work item.** Everything the team agreed on stays visible as work.

---

## Order and naming

`issues/NN-<slug>.md`, two digits, numbered in dependency order. Roots first, then outward — the same order the spec emits work items in.

**Nothing depends on a higher number, ever.** The filenames are the queue; there is no index file to correct them.

A dependency cycle, or an item depending on a later one, means the order can't be written. Stop, name the cycle, write nothing.

---

## The issue file

````markdown
# 03 — <title>

Spec: W4 · Depends on: 02

## Context

- <what's wrong today>
- <why this work exists>

## Task

<a sentence or two: what to build, the reason folded in>

## Decisions that bind this

- **<decision as a claim>.** <what it rules out>

## Conventions to follow

- <a pattern the repo already uses that this work follows>

## Done when

- <observable outcome>
- <observable outcome>

## Tests

- **Integration** — <case>
- **Unit** — <case>, <why this one needs isolating>

## Out of scope

- **<thing>** — <the issue that owns it, or why it's out>
````

**Omit any section with nothing in it.** A heading followed by nothing is noise an agent reads anyway. `Done when` is the exception that never legitimately empties — an issue with no observable outcome isn't ready to file.

### The header line

`Spec: W4 · Depends on: 02`. Several dependencies are comma-separated. None is `Depends on: —`.

### Context

Bullets only. Enough that someone who never saw the spec knows why this work exists. Two or three.

### Task

Prose, not a checklist. What the work is, with the reason folded in. `Done when` carries the outcomes; this carries the intent.

### Decisions that bind this

Only decisions that constrain *this* issue. State the claim, then what it forbids — *"Fees compute server-side. Don't add a client-side path, even temporarily."* A decision an agent can't violate from inside this issue doesn't belong here.

Carry no rejected alternatives and no reasoning about why the decision won. The agent needs the constraint, not the argument.

### Conventions to follow

What the repo already does that this work should match. One line each. No paths, no symbols.

### Done when

Observable outcomes. Not steps. Around five is the ceiling — more than that and the item wanted splitting.

### Out of scope

Only what someone doing this issue would plausibly wander into. Name the issue that owns it where one does.

---

## Tests

Follow the repo's own testing conventions first — its framework, its layout, its style. That's what exploring is for.

Where the repo has no convention to follow:

- **Integration over unit.** Test the behavior through the seam it's used at.
- **Unit tests only for logic worth stressing on its own** — complex string parsing, non-trivial calculation, anything with boundaries a caller can't easily reach.

Name cases, not counts. *"Refund with a fee returns the net amount"*, never *"add tests for the fee path"*.

---

## The QA issue

The spec's **Manual QA Plan** becomes the last issue, depending on every other one. It's the only place the assembled work gets checked, because no single issue proves it.

Write it for a person: what to click, what to look at, what should be true. Note in it that these checks are the natural seed for an end-to-end suite later.

When the spec reads `No manual QA required.`, file no QA issue.

---

## Open questions

The spec's **Open questions** section is not carried. Issues are written as though those questions were settled, and no issue warns about them.

---

## Before you save

Read the issues back and fix what you find:

- **An issue depending on a higher number.**
- **An issue consuming another's output without declaring the dependency.**
- **A leaked file path or symbol name.**
- **A vague `Done when`** — "works correctly", "handles errors", "as appropriate".
- **An issue with no `Done when`.**
- **A work item that vanished** in a split or a merge.
- **A padded section** kept when it had nothing in it.

Fix them silently. Then say what you fixed in one line: *"Fixed 2 things before saving: a path in 04, and 05 depended on 07."* Say so when there was nothing.

The same line reports any split, any merge, and any merge you skipped because you couldn't ask.

### Where it goes

`issues/` beside the spec, in the same directory.

**If `issues/` already exists, stop and ask.** The files may have been hand-edited or partly implemented, and nothing in the directory records which.

---

## Never implement

You produce the files. You do not write the code and you do not start on issue 01 because it looks small. Implementing is the user's call.
