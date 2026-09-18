# Spec skill

Turns a settled plan into one document an engineering team can validate before the work starts. It's also what gets read in the kickoff.

Sits after [probe](../probe/): probe settles the decisions, `spec` writes them up as contracts — schemas, endpoints, types, interfaces, call order, constraints — in the order they get built.

## Files

- **SKILL.md** — Skill definition.
- **example.md** — A filled-in spec. The target the skill writes toward.

## Installation

```sh
cp -r skills/spec ~/.claude/skills/spec
```

## How it works

`/spec path/to/doc.md` reads that document — a probe document, a PRD, meeting notes, a ticket, anything. With no path it uses the session.

It agrees a file path, explores the repo so the contracts use real names, then writes the document **one section at a time**. Each round: write to the file, show it, wait for the user to agree. Nothing moves on until they do.

```
R0   agree the file path
R1   Context + Out of scope
R2   Flow
R3   entry 1
...
Rn   Assumptions + Open questions + QA Plan
```

Seven sections, always in this order:

```
Context · Out of scope · Flow · Plan · Assumptions · Open questions · QA Plan
```

**Flow** is the diagrams — a column per boundary the work crosses. **Plan** is the numbered entries, each holding the contracts it's responsible for:

```
### 5 — Deliveries API

**Inputs & outputs**   route, headers, query, status codes
**Types**              what this entry defines
**Interfaces**         signatures and returns
**Callstack**          the call tree from this entry's entry point
**Constraints**        what must hold
```

Only the blocks that apply. A UI entry leads with a **Prototype** — a local HTML file beside the spec, in the repo's own colors, built and approved before its contracts are written.

## Design decisions

- **The document is contracts, not descriptions.** Schemas, signatures, payloads, call trees — written out in the repo's real idiom. An engineer who can read the language they're about to write can challenge it line by line.
- **Section by section, user in the loop.** A spec approved in one go gets approved without being read. One section per round, and nothing moves on until the user agrees.
- **Write first, then ask.** The skill never asks what a section should say. It writes it from the input and the repo, then asks whether it's right. A gap it can't fill becomes an Open question — asking would make it a second probe.
- **Don't invent decisions, do invent names.** A contract needs identifiers, so the skill names the class, the route, the type — and says which names it invented, so they can be corrected. A decision nobody made still goes to Assumptions or Open questions.
- **Constraints absorb behavior.** Input validation and system guarantees sit in one list, because a reader cares what breaks, not which category it fell into. This is where the old outcome bullets went.
- **Types are referenced, never inferred.** `Returns: 200: Array<WebhookDeliveryResource>`, `state: DeliveryState`. A type block that doesn't say where its type is used makes the reader guess.
- **Block order is definition before use**, with a Prototype first and Constraints last. Not a fixed list — the entry decides, the reader meets every name before it's referenced.
- **Blocks are adaptive.** A migration entry has a Schema and nothing else. No block is ever written with "None" in it, and new block kinds are allowed when the work has a contract the seven don't cover.
- **HTTP contracts use wire notation** even in a typed repo. The wire format has no language idiom.
- **Callstacks are per entry**, rooted at that entry's own entry point and stopping where the chain leaves it. They're also what discharges the rule that a mermaid diagram can't carry information alone — mermaid is raw source in a terminal, the callstacks are not.
- **UI entries get a prototype, offered first.** A local HTML file beside the spec, following the repo's existing colors and structure. Built and approved before the contracts, so the types are written to match what the user actually saw. One round, not a loop.
- **The flows are diagrams, never prose.** A narrated end state is the entries written twice, and the copy goes stale first. If a diagram needs a paragraph to be understood, the diagram is wrong.
- **Diagrams cap at eight participants or twelve nodes.** Past that they get split, not styled — a diagram nobody can read is worse than none, because it looks like the question was answered.
- **No rejected alternatives.** The document says what the team is doing. Arguing against options nobody picked is what the kickoff is for.
- **No length cap.** Big work gets a long document. The enemy is walls of text: no paragraph past three lines, and anything longer becomes a list, a table, a diagram, or code.
- **A worked example ships with the skill.** Rules describe good writing; an example shows it. This is the main defense against output that reads generated.
- **The numbering is the order.** Entries are emitted so nothing depends on a later one, and no entry points forward by number — a document where the numbers are wrong is one to fix, not to annotate.
- **One-way changes get their own entry, first.** A migration is the riskiest thing in the document; it doesn't hide inside an entry about something else.
- **A cycle means something is unnamed.** Two entries that need each other share a contract nobody wrote down. Name it, make it entry 1.
- **One review pass, at the end.** Section-local checks can't catch a forward dependency or two entries stating the same guarantee differently. The pass runs once on the finished document, and a reorder of already-approved entries is named explicitly rather than fixed silently.
- **Non-code work still fits.** A process or a team decision skips Flow and the contract blocks, and its entries are bullets. The loop runs the same way.
- **It never implements.** Output is the document, plus the prototype where a UI entry called for one.
