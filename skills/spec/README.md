# Spec skill

Turns decisions that are already settled into one document a team can agree on, sliced into work items ready to become issues.

Sits between [probe](../probe/) and whatever files your tickets: probe settles the decisions, `spec` writes them up and breaks the work apart, you agree on it, then the issues get filed.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/spec ~/.claude/skills/spec
```

## How it works

`/spec path/to/base_doc.md` reads that document. With no path it uses the current session, and with no probe behind it at all it synthesizes whatever was discussed.

It explores the repo on its own, writes the document, checks its own output, and saves — beside the probe document it read, or `docs/<timestamp>_<slug>/spec.md` when it came from a session.

The document is always the same seven sections:

```
Context · Decisions · Out of scope · Assumptions · Open questions · Work items · Manual QA Plan
```

## Design decisions

- **No interview.** probe owns that. The one thing `spec` stops for is a repo that contradicts something probe decided — then the ground the decision stood on is gone, and it asks instead of writing it down as settled.
- **The document is the review.** It doesn't confirm its findings halfway through; it writes the thing and lets the team react to it.
- **`Depends on` means "must come after".** If an item consumes what another produces, that's a dependency, whether or not it strictly blocks. The UI comes after the endpoint, no soft-ordering.
- **Items are emitted in dependency order**, roots first — the same rule probe uses when it refuses to ask a blocked question.
- **One-way changes get their own item, first.** A migration is the riskiest thing in the document; it doesn't hide inside an item about something else.
- **A cycle means something is unnamed.** Two items that need each other share a contract nobody wrote down. `spec` names it and makes it the first item, rather than merging them or hiding a stub.
- **`Touches` names areas, never paths.** A path that moved between writing and filing sends an agent confidently to the wrong place.
- **Rejected alternatives are kept**, one line each with the reason they lost. It's the part people come back to six weeks later.
- **Assumptions say what breaks if they're wrong.** An assumption with no consequence isn't worth a line.
- **It never implements.** Output is the document.
