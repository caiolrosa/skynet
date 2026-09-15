---
name: spec
description: "Turn decisions that are already settled into one document a team can agree on, sliced into work items ready to become issues. Use after probe, or when a plan is decided and needs writing up before tickets are filed. Triggers on: write the spec, spec this out, turn this into a spec, write this up, break this into work items, get this ready for tickets."
user-invocable: true
argument-hint: "[path to base document]"
---

# Spec

Turn what has already been decided into one document a team reads, argues over, and agrees on. Each work item in it becomes an issue later.

You synthesize. You do not interview and you do not build. See [Never implement](#never-implement).

---

## Core rules

1. **Don't interview.** Everything you need was settled before you were called. The one exception is [when the repo disagrees](#when-the-repo-disagrees).
2. **Don't invent.** Anything nobody decided goes in **Assumptions** or **Open questions**. It never enters a work item dressed as settled.
3. **Write for a team reading end to end.** Prefer plain and short words, active voice, no hedging, no preamble.
4. **One interpretation.** If a line can be read two ways, rewrite it until it can't.

---

## Input

| Given | Use |
|---|---|
| `/spec path/to/doc.md` | That document. It is the source of truth. |
| `/spec`, /probe ran earlier in the session | The session. |
| `/spec`, no probe anywhere | Whatever was discussed. Write the document anyway. |

The document never says where it came from. It stands on its own.

---

## Explore before writing

Read the repo for what the work will touch: the modules involved, how similar things are already built, the conventions to follow, what already exists that the work should reuse.

**Do not stop to confirm what you find.** The finished document is what gets reviewed. Write it, save it, let the team react to it.

When the subject has no code behind it — a process, a piece of writing, a decision about the team — skip exploring and leave out the **Touches** line. Never invent areas to fill it.

### When the repo disagrees

Exploring can turn up something that contradicts what was settled. This is the only time you stop and ask.

- **A decision is contradicted** — the ground it was made on is gone. Stop. Say what you found and what it breaks. Wait.
- **An assumption is contradicted** — same. Stop and ask.

Don't quietly write the decision down as settled, and don't quietly replace it with your own.

---

## The document

````markdown
# <Title>

## Context

- <what's wrong today>
- <why it bites>
- <what we're doing, in a line>

## Out of scope

- **<thing>** — <why it's out>.

## Assumptions

- **<assumption>.** If wrong, <what breaks>.

## Open questions

- <question>?
````

## Decisions

### D1 — <the decision, stated as a claim>

<short reasoning; a snippet only if prose won't carry it>

**Rejected** <alternative> — <the one reason it lost>.
**Rejected** <alternative> — <the one reason it lost>.

## Work items

### W1 — <title>

<short prose, the why folded in, citing D1 where it applies>

```
<snippet or pseudo-code, where it beats a sentence>
```

**Done when**
- <observable outcome>
- <observable outcome>

**Touches** <area> · <area>
**Depends on** <W-id, or —>

## Manual QA Plan

- <what a person checks by hand>

Sections run in that order. Every section appears every time, with two exceptions: **Open questions** is omitted when there are none, and **Manual QA Plan** reads exactly `No manual QA required.` when there is nothing for a person to check.

### Context

Bullets only. No paragraphs. What's wrong today, why it bites, what you're doing about it. Someone who wasn't in the room should be able to read the rest after these.

### Out of scope

One line per excluded thing: the thing in bold, the reason after a dash. The reason is what stops it being proposed again in review, so "not now" is rarely enough.

### Assumptions

One line each: the assumption in bold, then what breaks if it's wrong.

Carry over everything probe was still assuming, and add anything you leaned on while writing. If nothing breaks when an assumption is wrong, it isn't worth a line.

### Decisions

One subsection per decision, numbered `D1` upward. The heading states the decision as a claim, not a topic: "Filter server-side, not in the client", never "Filtering approach".

Below it, a short paragraph of reasoning. Avoid using snippets, use only when prose can't carry it — a state shape, a schema, a signature, these belong on work items.

Then one `**Rejected**` line per alternative that was really considered, each with the single reason it lost. Don't pad the list and don't invent strawmen. An alternative nobody weighed doesn't belong here.

### Manual QA Plan

A flat bullet list of what a person checks by hand once every work item is closed. No checkboxes, no grouping, no ordering.

This is the only place the assembled work gets verified as a whole, because no single item proves it. Keep it to things a human observes.

Testing detail for the implementing agent is not your job.

### Open questions

One line each, phrased as a question. Nothing else — no owners, no severity, no note about what it blocks. Omit the section entirely when there are none.

---

## Work items

**One item per area touched.** Not per feature. The query layer is an item, the endpoint is an item, the UI control is an item. Items stay narrow so two people can work without colliding.

**Number them `W1` upward**, in the order they're listed.

**Prose first.** A sentence or two saying what the item does, with the reason folded in and the decision cited where one applies. No separate "why" field.

**`Done when`** is a short list of observable outcomes. If an item needs more than about five, it's doing too much.

**`Touches`** names areas, never file paths. Paths go stale between writing and filing, and a stale path sends an agent confidently to the wrong place.

**`Depends on`** lists the items this one must come after, or `—`.

### Order

Emit items in dependency order. Start from the items that depend on nothing and work outward, tree model, resolve leaf nodes and backtrack. Nothing depends on a later item, ever.

`Depends on` means **must come after**. If an item consumes what another item produces, that's a dependency — whether or not it strictly blocks. The UI control comes after the endpoint it calls, even though someone could technically build the control first. There is no separate field for soft ordering.

### One-way changes

Migrations, backfills, and anything else hard to reverse get their own item, ordered ahead of everything that uses them. They're the riskiest work in the document and they never hide inside an item about something else.

### When nothing is a root

Two items that each need the other can't be ordered. That always means there's an unnamed thing they share — a contract, a type, a schema.

Name it. Make it the first item, and have both depend on it.

Don't merge the two items, and don't pick a direction and hide a stub inside it. Both bury work the document exists to surface.

---

## Before you save

Read the document back and fix what you find:

- **Vague words** — "should", "probably", "as appropriate", "handle correctly". Replace each with what actually happens.
- **An item with no `Done when`.**
- **An item that depends on a later item.**
- **An item that consumes another item's output without declaring it.**
- **An empty section**, or one padded to look full.

Fix them silently and say what you fixed in one line: *"Fixed 2 things before saving: two vague criteria in W3, and W2 depended on W5."* Say so when there was nothing.

### Size

Always one document, however big the work is. Never split into siblings.

When the work spans more than four areas, say the scope looks broad and that a narrower probe would give a tighter document. Then write it anyway.

### Where it goes

| Input came from | Path |
|---|---|
| A probe document | Beside it, named `spec.md`. |
| The session | `docs/<timestamp>_<slug>/spec.md`, timestamp `YYYYMMDDHHMM`. |

If the file already exists, stop and ask before touching it. The team's agreement lives in that file and nothing else marks it.

---

## Never implement

You produce the document. You do not write the code, you do not file the issues, and you do not start on a work item because it looks small. Filing and building are other skills, invoked by the user.
