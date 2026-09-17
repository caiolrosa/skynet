# Spec skill

Turns a settled plan into one document an engineering team can validate before the work starts. It's also what gets read in the kickoff.

Sits after [probe](../probe/): probe settles the decisions, `spec` writes them up as a design plus the ordered changes that get there.

## Files

- **SKILL.md** — Skill definition.
- **example.md** — A filled-in spec. The target the skill writes toward.

## Installation

```sh
cp -r skills/spec ~/.claude/skills/spec
```

## How it works

`/spec path/to/doc.md` reads that document — a probe document, a PRD, meeting notes, a ticket, anything. With no path it uses the session. With nothing at all it asks one round of up to four questions, then writes.

It explores the repo so the design names real things, writes the document, reads its own output back and fixes what it finds, then asks where to save.

Six sections, always in this order:

```
Context · Out of scope · Plan · Assumptions · Open questions · QA Plan
```

**Plan** opens with the flows as diagrams — a column per boundary the work crosses — then the numbered changes in dependency order. Each change is a title and what's true when it's done.

## Design decisions

- **Written to be argued with.** The reader is an engineer deciding whether the approach is wrong. A plan that names real components gets challenged; a plan of abstractions gets nodded at.
- **Rationale is a trailing clause**, not its own line and not a separate decisions section — and only where the outcome doesn't explain itself.
- **No rejected alternatives.** The document says what the team is doing, not what it isn't. Arguing against options nobody picked is what the kickoff is for.
- **The flows are diagrams, never prose.** A narrated end state is the entries written twice, and the copy goes stale first. If a diagram needs a paragraph to be understood, the diagram is wrong.
- **Reach for the artifact first.** Code, a schema, a diagram, a table — whatever carries the idea better than prose. Prose is the fallback, and it only wins on judgments: why a shape is that shape, what a constraint costs.
- **No length cap.** Big work gets a long document. The enemy is walls of text: no paragraph past three lines, and anything longer becomes a list, a table, a diagram, or code.
- **A worked example ships with the skill.** Rules describe good writing; an example shows it. This is the main defense against output that reads generated.
- **Diagrams cap at eight participants or twelve nodes.** Past that they get split, not styled — a diagram nobody can read is worse than none, because it looks like the question was answered.
- **Nothing relies on mermaid rendering.** It shows as raw source in half the places a spec gets pasted, so the same information is readable as text somewhere in the document.
- **It asks only when there's nothing to go on**, and then only once. With a document or a session in hand it asks nothing and turns the gaps into open questions.
- **A change entry is a title and a list of outcomes.** No description above them, no `Done when` label below them — describing the work and stating its outcome is the same sentence written twice, and the outcome is the version a reviewer can disagree with. Where the change produces an artifact, the artifact goes there instead.
- **The numbering is the order.** Entries are emitted so nothing depends on a later one, and there's no dependency field — a document where the numbers are wrong is one to fix, not to annotate.
- **One-way changes get their own entry, first.** A migration is the riskiest thing in the document; it doesn't hide inside an entry about something else.
- **A cycle means something is unnamed.** Two changes that need each other share a contract nobody wrote down. Name it, make it change 1.
- **It asks where to save.** No convention about where specs live survived contact with more than one repo.
- **It never implements.** Output is the document.
