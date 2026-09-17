---
name: spec
description: "Turn a settled plan into one document an engineering team can validate before the work starts. Use after probe, or when a plan is decided and needs writing up for review or a kickoff. Triggers on: write the spec, spec this out, turn this into a spec, write this up, plan this out, get this ready for review."
user-invocable: true
argument-hint: "[path to input document]"
---

# Spec

Write the document a team reads before the work starts. They read it to judge whether the approach is right, and they read it out loud in a kickoff.

Two readers, one document: someone who knows the area and wants to attack the design, and someone who has never seen it and needs to follow along. Both of them skim first.

You write. You do not build. See [Never implement](#never-implement).

---

## Core rules

1. **Don't invent.** Anything nobody settled goes in **Assumptions** or **Open questions**. It never appears inside the plan dressed as decided.
2. **One interpretation.** If a line can be read two ways, rewrite it until it can't.
3. **Design-level, but show the artifact.** Name the components, the contracts, and what moves between them — then, where the thing itself is small and concrete, write it out in the repo's own idiom instead of describing it. A migration, a schema, a type, a payload, a signature, a config block. Prose about a data structure is always longer than the structure and always vaguer.
4. **Nothing is read twice.** A reader who stops mid-paragraph to re-read it has hit a defect. See [Shape](#shape).
5. **[skills/readability.md](../readability.md) binds every line.** Its rules set the wording; this file sets the shape.

---

## Input

| Given | Use |
|---|---|
| `/spec path/to/doc.md` | That document. |
| `/spec`, a probe ran this session | The session. |
| `/spec`, something substantial was discussed | The discussion. |
| `/spec`, nothing to go on | [Ask one round](#when-there-is-nothing-to-go-on), then write. |

**Any document is the source of truth** — a probe document, a PRD, meeting notes, a ticket. No format check, no refusal. Read what's there and write from it.

The document never says where it came from. It stands on its own.

### Carrying a probe document over

| In the probe | In the spec |
|---|---|
| `⚠️ assuming` ledger | **Assumptions**, each with what breaks if it's wrong |
| "Left open" | **Open questions** |
| A tension the user closed with "take it as-is" | **Assumptions**, with the consequence spelled out |

**When there's a document or a session, ask nothing.** Its gaps become **Open questions**. Re-asking what the user already answered is the failure mode this rule exists to stop.

### When there is nothing to go on

One round through `AskUserQuestion`, four questions at most, on the things you would otherwise have to invent. Then write, whatever the answers were.

One round. Not two. Interviewing is [probe](../probe/SKILL.md)'s job, and a second round means you're doing it badly.

---

## Explore before writing

Read the repo for what the work touches: the modules involved, how similar things are already built, the conventions to follow, what already exists that the work should reuse.

This is what makes the design concrete enough to argue with. A plan that names real components gets challenged; a plan of abstractions gets nodded at.

**Don't stop to confirm what you find.** The finished document is what gets reviewed.

### When the repo disagrees

Exploring can turn up something that contradicts what was settled. This is the only time you stop.

Say what you found and what it breaks. Wait. Don't write the decision down as settled, and don't quietly replace it with your own.

### When there's no code behind it

A process, a piece of writing, a decision about the team. Skip exploring, skip **Flow**, and expect no artifacts — the entries are bullets.

---

## The document

````markdown
# <Title>

## Context

- <what's wrong today>
- <why it bites — only when that isn't obvious from the line above>

## Out of scope

- **<thing>** — <why it's out>.

## Plan

### Flow

```mermaid
---
title: <what this flow is>
---
<the solution as flows across boundaries — no prose>
```

### 1 — <title>

```<the repo's language>
<the artifact, where the change produces one>
```

- <otherwise, bullets: what is true once this is done>
- <one outcome each, with the reason as a trailing clause where it isn't obvious>

## Assumptions

- **<assumption>.** If wrong, <what breaks>.

## Open questions

- <question>?

## QA Plan

- <what a person checks by hand>
````

[example.md](example.md) is a filled-in spec. Read it before writing and match its texture — it is the target, not the template above.

**Section names and order are fixed.** The shape inside them is yours: prose, a list, a table, a diagram, code — whatever the content actually is.

**An empty section is omitted, not padded.** The two exceptions: **Plan** never legitimately empties, and **QA Plan** reads exactly `No manual QA required.` when there's nothing for a person to check.

### Context

What's wrong today, and why it bites when that isn't already obvious. Bullets.

**The approach doesn't go here.** It's the whole of **Plan**, and a one-line preview of it just gets contradicted by the real thing two screens down.

Someone who wasn't in the room reads these bullets and can follow the rest.

### Out of scope

One line each: the thing in bold, the reason after a dash.

The reason is what stops it being proposed again in the kickoff. "Not now" rarely does that. "Needs the new auth model, which lands next quarter" does.

### Assumptions

One line each: the assumption in bold, then what breaks if it's wrong.

If nothing breaks when it's wrong, it isn't an assumption worth a line. This is the section a room is best placed to shoot down, so make each one easy to disagree with.

### Open questions

One line each, phrased as a question. No owners, no severity, no note about what it blocks.

### QA Plan

A flat list of what a person checks by hand once every change has shipped. No checkboxes, no grouping, no ordering.

This is where the assembled work gets verified, because no single change proves it. Keep it to what a human observes. Test design for whoever implements is not your job.

---

## Plan

The flows, then the ordered changes.

**Reach for the artifact first.** If code, a schema, a diagram, or a table carries the idea better than prose or bullets, that's what goes in the entry. Never write a paragraph describing a shape instead of showing the shape.

| The idea is | Show |
|---|---|
| A structure — table, type, payload, config, interface | The thing itself, in the repo's idiom |
| A flow, a state machine, a call sequence across services | A mermaid diagram |
| A comparison along shared dimensions | A table |
| Anything else | Bullets, one idea each |

Prose is the fallback, not the default. It wins when the idea is a judgment — why a shape is that shape, what a constraint costs — and loses everywhere else.

### Flow

The solution as diagrams, opening the section. One or two, no more — and each inside the size limit in [Diagrams](#size).

**Every diagram carries a title**, in a `---` frontmatter block inside the fence, never as a line of markdown above it. It renders with the picture and travels with it when someone exports the diagram alone.

**Columns are boundaries.** A participant per process, service, store or external system the work crosses — the HTTP request, the worker, the queue, the third party. The value of the picture is seeing what hands off to what, and where something stops being ours.

Usually a `sequenceDiagram` for how a request moves, and a `stateDiagram-v2` where something has a lifecycle. Both, when they answer different questions.

**No prose.** Not a sentence above the diagram, not a paragraph below it. A narrated end state is the entries written twice, and the copy goes stale first. If the diagram needs a paragraph to be understood, the diagram is wrong — fix it or cut it.

Skip the section when the work has no flow across a boundary. A change that lives in one process doesn't get a diagram of one column.

### Entries

One entry per thing that can be built and judged on its own. Numbered from `1`, in the order they happen.

**An entry is a title and what's true when it's done.** Nothing else.

Bullets, one observable outcome each. That list is the whole body — no description above it, no `Done when` label below it. Describing the work and stating its outcome is the same sentence written twice, and the outcome is the version a reviewer can disagree with.

```
### 2 — Publish writes a delivery row

- Publishing an event inserts a pending delivery row per configured endpoint.
- The request path makes no outbound HTTP call.
- The payload stored matches what the old inline path sent, byte for byte.
```

**Where an artifact carries it better, that's the entry** — a migration, a type, a config block, a state diagram. The artifact is the outcome, and restating it as bullets underneath says nothing new.

**Reasoning is a trailing clause, never its own line.** *"Two workers never send the same delivery twice. The merchant has no idempotency key to dedupe on."* Where the outcome explains itself, there's no clause.

**No rejected alternatives.** The document says what the team is doing. An argument against something nobody is doing is the reader's problem to follow, and the kickoff is where it gets raised anyway.

**Constraints that span entries** — what's frozen, what can't be undone, what two entries both have to honour — go on the entry that creates them. They have no section of their own.

**Nothing repeats the flow.** If a diagram at the top already shows it, the entry states its outcome and stops.

Past five or six outcomes, the entry wanted splitting.

### Order

The numbering is the order. Entries are emitted so that **nothing depends on a later one**, roots first and outward from there.

Dependency means *must follow*, not *is blocked by*. If an entry consumes what another produces, it comes after it — even when someone could technically build them in either order. The UI control comes after the endpoint it calls.

There is no field for this. A reader who needs the order reads the numbers, and a document where the numbers are wrong is a document to fix, not to annotate.

### One-way changes

Migrations, backfills, anything hard to reverse: **its own entry, ordered ahead of everything that uses it.** It's the riskiest work in the document and it never hides inside an entry about something else.

### When nothing is a root

Two entries that each need the other can't be ordered. That always means they share something nobody has named — a contract, a type, a schema.

Name it. Make it entry 1. Both come after it.

Don't merge the two, and don't pick a direction and bury a stub inside it. Both hide work the document exists to surface.

### Size

One document, however big the work is. Six areas gets a long document; write it. No scope warning, no splitting into siblings.

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

Mermaid renders in some places and shows as raw source in others — a terminal, a kickoff slide, half the tools people paste into. Somewhere in the document the same thing has to be readable as text. The entries usually do it, so check there before adding a paragraph beside the diagram.

---

## Shape

Length is not the enemy. Walls of text are.

- **No paragraph past three lines.** Longer than that becomes a list, a table, a diagram, or a code block.
- **Never describe in prose what you can show as code.** A table, a payload, a type, a config. Write it in the repo's idiom and keep the prose for the part code can't say — why it's shaped that way.
- **A list needs three or more parallel items.** One item is a sentence. Two usually are too.
- **Tables only for comparing along shared dimensions.** Not for laying out prose.
- **Code block when prose would run three lines or more.** Pseudo-code counts.
- **One idea per sentence.** Two commas holding a sentence together means it's two sentences.
- **No sentence that only introduces the next one.**
- **Nothing reads "as mentioned above".** Each section stands where the reader lands on it.
- **Never restate a heading in its first line.**

---

## Before you save

Read the document back and fix what you find:

- A paragraph past three lines.
- A change that depends on a later one.
- A bullet that restates what the entry already shows — reasoning paraphrasing the artifact, an outcome nobody would skip checking.
- Bullets or prose doing a job an artifact would do better.
- A change that consumes another's output without declaring it.
- Vague words — "should", "probably", "as appropriate", "handle correctly". Replace each with what actually happens.
- An empty section, or one padded to look full.
- A diagram carrying something the text doesn't.
- Prose describing a concrete artifact that could just be shown.
- Reasoning that restates the change instead of justifying it.

Fix them silently, then say what you fixed in one line: *"Fixed 2 things before saving: a vague outcome in change 3, and 2 came after 5."* Say so when there was nothing.

### Where it goes

**Ask where to save it.** Propose `spec.md` in the directory the input document came from, or the working directory when there wasn't one.

**If a file is already there, stop and ask.** The team's agreement lives in that file and nothing else marks it.

---

## Never implement

You produce the document. You don't write the code, you don't file issues, and you don't start on change 1 because it looks small. Those are other skills, invoked by the user.
