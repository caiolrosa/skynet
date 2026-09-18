---
name: spec
description: "Turn a settled plan into one document an engineering team can validate before the work starts. Use after probe, or when a plan is decided and needs writing up for review or a kickoff. Triggers on: write the spec, spec this out, turn this into a spec, write this up, plan this out, get this ready for review."
user-invocable: true
argument-hint: "[path to input document]"
---

# Spec

Write the document a team reads before the work starts. They read it to judge whether the approach is right, and they read it out loud in a kickoff.

The document is contracts. Schemas, endpoints, types, interfaces, call order, constraints — written out, not described. Someone implementing from it should be arguing about the design, never guessing at a shape.

You write it one section at a time, and the user approves each before you write the next. See [The loop](#the-loop).

You write. You do not build. See [Never implement](#never-implement).

---

## Core rules

1. **Don't invent decisions.** Anything nobody settled goes in **Assumptions** or **Open questions**. It never appears inside the plan dressed as decided.
2. **Do invent names.** A contract needs identifiers. Name the class, the method, the type, the route — then say you did, so the user can correct it.
3. **One interpretation.** If a line can be read two ways, rewrite it until it can't. Contracts exist to close readings that prose leaves open.
4. **Show the artifact.** A schema, a payload, a signature, a call tree. Prose about a shape is always longer than the shape and always vaguer.
5. **[skills/readability.md](../readability.md) binds every line.** Its rules set the wording; this file sets the shape.

---

## Input

| Given | Use |
|---|---|
| `/spec path/to/doc.md` | That document. |
| `/spec`, a probe ran this session | The session. |
| `/spec`, something substantial was discussed | The discussion. |
| `/spec`, nothing to go on | One round of questions on the subject itself, then start the loop. |

**Any document is the source of truth** — a probe document, a PRD, meeting notes, a ticket. No format check, no refusal. Read what's there and write from it.

The document never says where it came from. It stands on its own.

### Carrying a probe document over

| In the probe | In the spec |
|---|---|
| `⚠️ assuming` ledger | **Assumptions**, each with what breaks if it's wrong |
| "Left open" | **Open questions** |
| A tension the user closed with "take it as-is" | **Assumptions**, with the consequence spelled out |

---

## The loop

One section per round. Write it to the file, show it, ask. Only move on once the user agrees.

```
R0   agree the file path
R1   Context + Out of scope
R2   Flow
R3   entry 1
R4   entry 2
...
Rn   Assumptions + Open questions + QA Plan
```

**Agree the path before writing anything.** Propose `spec.md` in the directory the input document came from, or the working directory when there wasn't one. The file exists from R1 and grows — the user can open it in an editor while you work. If a file is already there, stop and ask.

**Never ask what a section should say.** Write it from the input and the repo, then ask whether it's right. A gap you can't fill from either becomes an **Open question**, not a question to the user. Interviewing is [probe](../probe/SKILL.md)'s job.

**Show what you wrote.** The file is on disk, but the user validates in chat. Print the section, then one line naming what's next and asking for changes.

```
➡️ Next: entry 3 — Retry worker. Types, Interfaces, Callstack, Constraints.

Anything to change, or shall I continue?
```

**Flag what you invented.** Names, numbers, shapes the input never specified — say so in a line under the section. A reviewer who can't tell your inventions from their decisions approves both by accident.

**A UI entry starts with its prototype.** See [Prototypes](#prototypes).

### Reopening

When the user changes an approved entry, everything built on it is suspect.

Name what it invalidates, rewrite those entries, and re-validate each one. Never leave an entry standing on a contract that changed under it.

> Entry 1's `state` enum changed. That invalidates entry 5 — it filters on it — and entry 7's `DeliveryRow`. Rewriting 5.

---

## Explore before writing

Read the repo for what the work touches: the modules involved, how similar things are already built, the naming conventions, what already exists that the work should reuse.

This is what supplies the idiom. A spec that names `Webhooks::Dispatcher#process_due` gets challenged; a spec that says "a worker processes pending items" gets nodded at.

**Don't stop to confirm what you find.** The loop is where it gets confirmed.

### When the repo disagrees

Exploring can turn up something that contradicts what was settled. This is the one thing that stops the loop mid-round.

Say what you found and what it breaks. Wait. Don't write the decision down as settled, and don't quietly replace it with your own.

### When there's no code behind it

A process, a piece of writing, a decision about the team. Skip exploring, skip **Flow**, and expect no contract blocks — entries are bullets. The loop still runs, section by section.

---

## The document

````markdown
# <Title>

## Context

- <what's wrong today>
- <why it bites — only when that isn't obvious from the line above>

## Out of scope

- **<thing>** — <why it's out>.

## Flow

```mermaid
---
title: <what this flow is>
---
<the solution as flows across boundaries — no prose>
```

## Plan

### 1 — <title>

**Types**

```<the repo's language>
<the type>
```

**Interfaces**

```<the repo's language>
<signature>  # => <return type>
```

**Callstack**

```
-> <entry point>
  -> <what it calls>
```

**Constraints**

- <what must hold, with the reason as a trailing clause where it isn't obvious>

## Assumptions

- **<assumption>.** If wrong, <what breaks>.

## Open questions

- <question>?

## QA Plan

- <what a person checks by hand>
````

[example.md](example.md) is a filled-in spec. Read it before writing and match its texture — it is the target, not the template above.

**Section names and order are fixed.** An empty section is omitted, not padded. The two exceptions: **Plan** never legitimately empties, and **QA Plan** reads exactly `No manual QA required.` when there's nothing for a person to check.

### Context

What's wrong today, and why it bites when that isn't already obvious. Bullets.

**The approach doesn't go here.** It's the whole of **Plan**, and a one-line preview of it just gets contradicted by the real thing two screens down.

### Out of scope

One line each: the thing in bold, the reason after a dash.

The reason is what stops it being proposed again in the kickoff. "Not now" rarely does that. "Needs the new auth model, which lands next quarter" does.

### Assumptions

One line each: the assumption in bold, then what breaks if it's wrong.

If nothing breaks when it's wrong, it isn't an assumption worth a line. This is the section a room is best placed to shoot down, so make each one easy to disagree with.

### Open questions

One line each, phrased as a question. No owners, no severity, no note about what it blocks.

Writing contracts surfaces these. A shape you couldn't close without guessing is an open question, not a guess.

### QA Plan

A flat list of what a person checks by hand once every change has shipped. No checkboxes, no grouping, no ordering.

This is where the assembled work gets verified, because no single change proves it. Keep it to what a human observes. Test design for whoever implements is not your job.

---

## Flow

The solution as diagrams, one or two, no more. Each inside the size limit in [Diagrams](#size).

**Every diagram carries a title**, in a `---` frontmatter block inside the fence, never as a line of markdown above it. It renders with the picture and travels with it when someone exports the diagram alone.

**Columns are boundaries.** A participant per process, service, store or external system the work crosses. The value of the picture is seeing what hands off to what, and where something stops being ours.

Usually a `sequenceDiagram` for how a request moves, and a `stateDiagram-v2` where something has a lifecycle. Both, when they answer different questions.

**No prose.** Not a sentence above the diagram, not a paragraph below it. If the diagram needs a paragraph to be understood, the diagram is wrong — fix it or cut it.

Skip the section when the work has no flow across a boundary. A change that lives in one process doesn't get a diagram of one column.

---

## Plan

The numbered entries. One per thing that can be built and judged on its own, numbered from `1`, in the order they happen.

**An entry is a title and its contracts.** No description above them, no outcome bullets below them.

### Blocks

| Block | Holds |
|---|---|
| **Prototype** | A link to the prototype file, and one line on what it demonstrates |
| **Schema** | Tables and columns, as a migration or DDL in the repo's idiom |
| **Inputs & outputs** | An HTTP contract: route, headers, query, body, status codes |
| **Types** | The types this entry defines |
| **Interfaces** | Signatures and what they return |
| **Callstack** | The call tree from this entry's own entry point |
| **Constraints** | What must hold — validation, guarantees, invariants |

**Only the blocks that apply.** A migration entry has a Schema and nothing else. A frontend entry has no Schema and no HTTP contract of its own. Never write a block with "None" in it.

**Add a block when the work needs one.** The seven above are the common set, not a closed list. Config, permissions, events, feature flags — if it's a contract, it gets a block.

**Order is definition before use**, with two fixed positions: a **Prototype** block leads the entry, and **Constraints** closes it. Everything between goes in whatever order lets a reader meet a name before it's referenced.

**Label with `**Bold**`, then a fence.** Never a fourth-level heading — `## Plan` → `### 3 — …` → `#### Types` is a level too deep, and a seven-entry spec grows an unreadable table of contents.

### Types are referenced, never inferred

A type block that doesn't say where its type is used makes the reader guess. Reference it by name from wherever it appears.

```
Returns:
  - 200: { data: Array<WebhookDeliveryResource> }
Query:
  - state: DeliveryState                       // optional
```

The same holds for a column (`t.string :state, null: false  # DeliveryState`), a return value (`#deliver(delivery)  # => DeliveryResult`), and a field inside another type.

A type used in two entries is defined in the earlier one and referenced by name in the later. It is never written twice.

### Notation

**The repo's real idiom.** A real migration, real Ruby or TypeScript, the repo's own naming. Not pseudo-code — a spec an engineer can read in the language they're about to write is one they can challenge line by line.

**Two exceptions:**

- **HTTP contracts use wire notation.** The wire format has no language idiom. Route, headers, query, body, status codes, as plain text.
- **No repo to read from** — a greenfield project, a design with no code yet. Fall back to pseudo-code: `<column>: <type>, <constraints>`, `create_delivery(<attributes>): <returns>`.

### Callstacks

The call tree from this entry's own entry point — an endpoint, a job, a component. One root, indented by call depth.

```
-> POST /v1/webhook_deliveries/:id/replay
  -> Api::V1::WebhookDeliveryReplaysController#create
    -> authenticate_merchant!
    -> Webhooks::Replay.call
```

**Only the depth this entry owns.** A chain that continues into another entry's code stops at the call. The other entry has its own root.

The callstacks are also what makes the **Flow** diagrams safe — see [Diagrams never carry information alone](#diagrams-never-carry-information-alone).

### Constraints

What must hold. Input validation, system guarantees and invariants in one list, because the reader doesn't care which is which — they care what breaks if it's violated.

```
- Only a "dead" delivery can be replayed. "pending" and "delivered" both return 409 —
  one would double-send, the other resends what the merchant already has.
- A delivery belonging to another merchant returns 404, not 403. A 403 confirms the id exists.
```

**Reasoning is a trailing clause, never its own line.** Where the constraint explains itself, there's no clause.

**Don't restate the artifact.** A constraint saying "the endpoint returns 409 on a bad state" when the I/O block already lists 409 is noise. The constraint is what the block can't show — *which* states, and why.

Past six or seven constraints, the entry wanted splitting.

### Order

The numbering is the order. Entries are emitted so that **nothing depends on a later one**, roots first and outward from there.

Dependency means *must follow*. If an entry consumes a type, table or endpoint another produces, it comes after it. The UI component comes after the endpoint it calls.

There is no field for this. A reader who needs the order reads the numbers.

**Never point forward.** An entry that says "until entry 4's limit" has a dependency the numbering already denies. State what this entry owns and stop.

### One-way changes

Migrations, backfills, anything hard to reverse: **its own entry, ordered ahead of everything that uses it.** It's the riskiest work in the document and it never hides inside an entry about something else.

### When nothing is a root

Two entries that each need the other can't be ordered. That always means they share something nobody has named — a type, a schema, a contract.

Name it. Make it entry 1. Both come after it.

Don't merge the two, and don't pick a direction and bury a stub inside it. Both hide work the document exists to surface.

### No rejected alternatives

The document says what the team is doing. An argument against something nobody is doing is the reader's problem to follow, and the kickoff is where it gets raised anyway.

---

## Prototypes

An entry with a UI surface gets a prototype: a single HTML file beside the spec, opened in a browser.

**Build it before the entry's contracts.** You see the thing, then the types and interfaces are written to match what was approved. On a UI entry the round runs:

```
1. offer the prototype
2. build spec.prototype.html
3. user opens it, revises or accepts
4. write Types, Interfaces, Callstack, Constraints from it
5. show the entry, ask
```

**Offer, don't assume.** Declining is normal and costs nothing — the entry then starts at Types like any other.

**Follow the repo's design.** Read its tokens, its Tailwind config, its existing components, and use those colors and that structure. With nothing to read, keep it unstyled and say so rather than inventing a look.

**Static fixtures, no network.** The prototype demonstrates states and interaction — a filter that filters, a button that appears only in one state. It calls nothing.

**One round, not a loop.** The user opens it, says what's wrong once, you fix it once. A prototype that needs four rounds is a design question the spec hasn't settled.

**It appears in the entry as a `**Prototype**` block**, first, with a relative link and one line on what it shows.

```markdown
**Prototype**

[spec.prototype.html](spec.prototype.html) — the list at three states, the filter, and the
replay button appearing only on a dead row. Static fixtures, no network.
```

---

## Diagrams

Mermaid, whenever a picture carries the idea better than the words would. Which kind:

| The idea | Diagram |
|---|---|
| A call sequence crossing services | `sequenceDiagram` |
| Something with states and transitions | `stateDiagram-v2` |
| Data moving across components | `flowchart` |
| A change to how layers stack | `flowchart`, top to bottom |

Not for a single component. Not to restate a list that was already clear.

### Size

**Eight participants, or twelve nodes. Past that, split it.**

A diagram nobody can read is worse than no diagram — it looks like the question was answered. Two diagrams answering one question each always beat one answering both, and splitting beats every styling trick below.

### Keeping them readable

| Lever | Syntax | What it buys |
|---|---|---|
| Title it | `---`<br>`title: …`<br>`---` inside the fence | Names the flow, and travels with the image |
| Group participants | `box Ours` … `end` | Ours vs theirs, visually |
| Number the steps | `autonumber` | "step 7" becomes citable in the kickoff |
| Group nodes | `subgraph` | Boundaries become visible boxes |
| Kill mirrored actors | `%%{init: {"sequence": {"mirrorActors": false}}}%%` | Drops the duplicate actor row at the bottom |
| Go wide | `flowchart LR` over `TD` | Wide beats tall on a slide |
| Dim what isn't ours | `classDef external fill:#eee,stroke:#999` | Third parties recede |
| Detail off the arrows | `Note over W,M: …` | Arrow labels stay short |

**Don't reach for `defaultRenderer: elk`.** It lays out dense graphs far better and GitHub doesn't ship it, so the diagram breaks exactly where most people read the spec.

### Diagrams never carry information alone

Mermaid renders in some places and shows as raw source in others — a terminal, a kickoff slide, half the tools people paste into.

**The callstacks discharge this.** Every boundary a Flow diagram crosses appears in some entry's callstack or interface list, in text. Check that before saving; don't add a paragraph beside the diagram.

---

## Shape

Length is not the enemy. Walls of text are.

- **No paragraph past three lines.** Longer than that becomes a list, a table, a diagram, or a code block.
- **Never describe in prose what you can show as code.** Keep the prose for the part code can't say — why it's shaped that way.
- **A list needs three or more parallel items.** One item is a sentence. Two usually are too.
- **Tables only for comparing along shared dimensions.** Not for laying out prose.
- **One idea per sentence.** Two commas holding a sentence together means it's two sentences.
- **No sentence that only introduces the next one.**
- **Nothing reads "as mentioned above".** Each section stands where the reader lands on it.
- **Never restate a heading in its first line.**

---

## Before you save

One pass over the finished document, after the last round. Read it back and fix what you find:

- An entry that depends on a later one, or points forward by number.
- A type defined in two entries, or referenced nowhere.
- A constraint restating what the block above it already shows.
- Two entries stating the same guarantee differently — the contradiction prose hides and contracts surface.
- Bullets or prose doing a job an artifact would do better.
- A paragraph past three lines.
- Vague words — "should", "probably", "as appropriate", "handle correctly". Replace each with what actually happens.
- An empty section, or one padded to look full.
- A boundary in a Flow diagram that no callstack or interface mentions.

Fix them silently, then say what you fixed in one line: *"Fixed 2 things before saving: a vague constraint in entry 4, and entry 3 pointed forward to entry 4's limit."* Say so when there was nothing.

**A reorder gets named.** Reordering entries the user already approved is the one fix that changes something they signed off on, so the line says which moved and why: *"moved 5 before 2 — 2 consumes the type 5 defines."*

---

## Never implement

You produce the document and, where a UI entry calls for one, the prototype beside it. You don't write the production code, you don't file issues, and you don't start on entry 1 because it looks small. Those are other skills, invoked by the user.
