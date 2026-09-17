---
name: issues
description: "Turn agreed work into one markdown file per unit of it, each standalone enough that an agent implements it without reading the spec. Use after spec, when the document is settled and the work needs slicing into files an agent picks up one at a time. Triggers on: file the issues, break this into issues, turn the spec into issues, generate the issues, slice this into tickets."
user-invocable: true
argument-hint: "[path to spec document]"
---

# Issues

Turn agreed work into one file per unit of it. Each file is a briefing: an agent opens it, builds it, and never opens the spec. A human opens it and can tell whether the right thing is about to be built.

You slice and brief. You do not interview and you do not build. See [Never implement](#never-implement).

---

## Core rules

1. **The source document is the source of truth.** Don't re-decide what it settled. Don't add what it didn't ask for.
2. **One issue, one agent, one sitting.** Never write a line that assumes the reader has anything else open.
3. **Show the artifact, never the location.** A migration, a type, a payload, a signature — written out in the repo's idiom. The names inside it are the artifact. A file path is a pointer, it goes stale between filing and implementing, and a stale one sends an agent confidently to the wrong place.
4. **More concrete than the spec, never more decided.** Detail the spec left to the implementer belongs here. A decision the spec didn't make belongs nowhere.
5. **[skills/readability.md](../readability.md) binds every line.** Its rules set the wording; this file sets the shape.

---

## Input

| Given | Use |
|---|---|
| `/issues path/to/spec.md` | That document. |
| `/issues`, a spec was written this session | That document. |
| `/issues`, something substantial was discussed | The discussion. |
| `/issues`, nothing to go on | [Ask one round](#when-there-is-nothing-to-go-on), then write. |

A spec is the best input and not the only one. Slice whatever is there — a PRD, a probe document, a ticket, a conversation.

### When there is nothing to go on

One round through `AskUserQuestion`, four questions at most, on the things you would otherwise have to invent. Then write, whatever the answers were.

One round. Not two. Interviewing is [probe](../probe/SKILL.md)'s job, and a second round means you're doing it badly.

---

## Explore before writing

Read the repo for how this kind of work is already done: the layering, the naming, the shape of similar things, how tests are written. This is what makes an issue implementable without a spec beside it.

Carry patterns, never locations. *"Strategies register themselves at module load, as the refund one does"* survives a refactor. *"Edit the strategy file"* doesn't.

**Skip exploring entirely** when the work has no code behind it — a process, a piece of writing, a decision about the team. Those issues carry no artifact, and their **Constraints** carry only what the team decided.

### When the repo disagrees

Exploring can turn up something that contradicts a decision already made. That is the only time you stop and ask.

Say what you found and what it breaks. Wait. Don't file the issue with a note attached — nothing in an issue file warns an agent off, so a contradicted decision reaches it as fact.

---

## Slicing

Work arrives as entries in a document. Those entries are material, not the slicing — cut by area, then by what can be verified on its own.

### One area, one issue

A migration or backfill, a request path, a background worker, a frontend view, an infrastructure change. Each is its own issue, **always**.

**A shared code dependency never merges two areas.** It becomes a `Depends on` line, and whatever the dependent side needs — the payload, the signature, the states — is written out in full inside its **Constraints**. Two files repeating a contract beats one file that can't be verified in halves.

### Merging inside one area

Two entries in the same area become one issue only when neither can be verified without the other. A worker's backoff schedule and the state it backs off into are one issue. Two endpoints are two issues, however similar.

When in doubt, don't merge. The source document's granularity is the floor.

### One-way changes come first

Migrations, backfills, anything hard to reverse: its own issue, ordered ahead of everything that uses it. It's the riskiest work in the set and it never hides inside an issue about something else.

### Never drop work

Everything the team agreed on stays visible as work. There is no cap on how many issues that produces — twenty issues on big work is twenty files, filed without comment.

---

## Order and naming

`issues/NN_snake_case_title.md`. Two digits, zero-padded, numbered in dependency order. Roots first, then outward.

```
issues/01_delivery_tables.md
issues/02_publish_writes_delivery_row.md
issues/03_retry_worker.md
```

**Nothing depends on a higher number, ever.** The filenames are the queue; there is no index file to correct them.

A dependency cycle, or an issue that needs a later one, means the order can't be written. Stop, name the cycle, write nothing. Two issues that each need the other share something nobody has named — a contract, a type, a schema. That missing thing is the fix, and it isn't yours to invent.

---

## The issue file

````markdown
# 03 — <title>

Depends on: 02

## Context

- <what's wrong today>
- <why this work exists>

## Build

<a line or two: what to build, the reason folded in>

```<the repo's language>
<the artifact, where the work produces one>
```

## Constraints

- **<claim>.** <what it rules out>

## Done when

- <observable outcome>

## Tests

- **Integration** — <case as behavior>: <the input or boundary that makes it bite>

## Out of scope

- **<thing>** — <the issue that owns it, or why it's out>
````

[example/](example/) holds a filled-in set, sliced from [spec/example.md](../spec/example.md). Read it before writing and match its texture — it is the target, not the template above.

**Omit any section with nothing in it.** A heading followed by nothing is noise an agent reads anyway. `Done when` is the exception that never legitimately empties — an issue with no observable outcome isn't ready to file.

### The header line

`Depends on: 02`, comma-separated for several. Omit the line entirely when there are none.

No reference back to the source document. The file stands alone, and a pointer into a document that gets re-sliced is a pointer that lies.

### Context

Two or three bullets. Enough that someone who never saw the spec knows why this work exists.

### Build

A line or two of intent, then the artifact. What the work is, with the reason folded in.

**Reach for the artifact first.** Prose describing a shape is always longer than the shape and always vaguer.

| The idea is | Show |
|---|---|
| A structure — table, type, payload, config, interface | The thing itself, in the repo's idiom |
| A lifecycle the issue owns | One mermaid diagram |
| A comparison along shared dimensions | A table |
| Anything else | The two lines of intent, and stop |

**At most one diagram per issue, and only for a lifecycle** — states and transitions the issue is responsible for. Never a re-draw of a flow the spec already showed. Cap it at eight participants or twelve nodes, and make sure the same facts are readable as text elsewhere in the file: issues get read in terminals, where mermaid is raw source.

### Constraints

Everything that binds this work, in one list, whatever it came from. A decision the team made and a pattern the repo already uses constrain the agent identically, and where a rule came from changes nothing about honoring it.

State the claim, then what it rules out:

```
- **Fees compute server-side.** No client-side path, not even temporarily.
- **Strategies register themselves at module load,** as the refund one does.
```

Only what constrains *this* issue. A rule the agent can't violate from inside it doesn't belong.

Carry no rejected alternatives and no argument for why a decision won. The agent needs the constraint, not the debate.

### Done when

Observable outcomes. Not steps.

**No ceiling.** An issue is as big as its area of work, and the slicing rules decide the cuts, never the length of this list.

A migration's outcomes are the schema existing and the migration running clean in both directions. That's a complete issue.

### Out of scope

Only what someone doing this issue would plausibly wander into. Name the issue that owns it where one does.

---

## Tests

Follow the repo's own testing conventions first — its framework, its layout, its style. That's what exploring is for.

Where the repo has no convention to follow:

- **Integration over unit.** Test the behavior through the seam it's used at.
- **Unit tests only for logic worth stressing on its own** — complex parsing, non-trivial calculation, boundaries a caller can't easily reach.

**Name the case as behavior, then give the input or boundary that makes it bite.**

```
- **Integration** — a sixth failure moves the delivery to dead: attempt_count 5, endpoint returns 500.
- **Unit** — the backoff schedule: attempt 0 gives 1m, attempt 5 gives 48h.
```

Not test code, and never a bulk suite. The agent writes one test at a time; this section stops it guessing which boundary was meant.

**Omit the section when nothing runnable exists** — a migration, a config file, a document.

---

## The QA issue

The source document's **QA Plan** becomes the last issue, depending on every other one. It's the only place the assembled work gets checked, because no single issue proves it.

Write it for a person: what to do, what to look at, what should be true. Note in it that these checks are the natural seed for an end-to-end suite later.

When the document reads `No manual QA required.`, file no QA issue.

---

## Open questions

The source document's **Open questions** are not carried. Issues are written as though those questions were settled, and no issue warns about them.

A warning in a file an agent reads alone is a warning an agent ignores. If an open question genuinely blocks the work, it belongs in the document, not in a file someone is about to build from.

---

## Before you save

Read the issues back and fix what you find:

- **An issue depending on a higher number.**
- **An issue consuming another's output without declaring the dependency.**
- **A contract referenced but not written out** in the issue that depends on it.
- **A leaked file path,** or a pointer telling the agent where to edit.
- **Two areas in one issue** — a migration inside a feature, a view inside an endpoint.
- **A vague `Done when`** — "works correctly", "handles errors", "as appropriate".
- **An issue with no `Done when`.**
- **Work that vanished** in a merge.
- **A padded section** kept when it had nothing in it.
- **A diagram that redraws the spec's flow** instead of a lifecycle the issue owns.

Fix them silently. Don't report them.

### What you print

The coverage map, one line per file, and nothing else:

```
01_delivery_tables          ← spec 1
02_publish_writes_row       ← spec 2
03_retry_worker             ← spec 3, 4
```

That map is the only trace of how the work was cut, and it's read at the one moment someone can act on it.

### Where it goes

`issues/` beside the source document, in the same directory.

**If `issues/` already exists, stop and ask.** The files may have been hand-edited or partly implemented, and nothing in the directory records which.

**There is no append mode.** A rerun after the document gains an entry hits the same stop — renumbering would repoint issues an agent is already building.

---

## Never implement

You produce the files. You do not write the code and you do not start on issue 01 because it looks small. Implementing is the user's call.
