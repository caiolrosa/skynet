# Impl skill

Builds the work in a briefing, one test at a time, then hands the diff to code review. It never commits.

Fourth in the chain: [probe](../probe/) settles the decisions, [spec](../spec/) writes them up, [issues](../issues/) slices them into briefings, `impl` builds one. It ends by calling [cr](../cr/), and it never commits.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/impl ~/.claude/skills/impl
```

## How it works

`/impl issues/03_thing.md` reads that document. A spec, a set of notes or a paragraph in chat works the same way — it reads whatever it's handed for the outcomes, the test cases and the constraints, wherever those sit in the text. Nothing has to be under a particular heading. Where the briefing states none of them, it derives what it can, says in one line what it derived, and builds.

Before writing, it reads the repo's standards docs, nearby code of the same kind, how the repo verifies, and `git status` — so anything already dirty stays out of what it reports at the end.

Then the loop, one slice at a time: one failing test, the smallest code that passes it, a fast check, repeat. Per slice it runs the single test file and a typecheck; once at the end, the full suite and lint.

When the work is verified, it sends the briefing's path to `cr` in a sub-agent and waits. It applies the findings it agrees with, re-verifies, and reports only what's left to decide: the findings it did not apply, in full. What it applied and what the diff already satisfied are counted in one line, never listed, and an axis that didn't run says so.

It leaves the tree dirty and names the files it touched.

## Design decisions

- **No required format.** It reads for outcomes, cases and constraints rather than for headings, so an issue file, a spec section and a paragraph in chat all work. A skill that breaks when a heading is renamed is coupled to the wrong thing.
- **It asks only when blocked.** `probe`, `spec` and `issues` exist to settle decisions upstream. An implement step that re-asks scope, approach and error handling makes the whole chain pointless.
- **No language context files.** The old version shipped six `langs/*.md` guides. The repo's standards docs and the conventions the briefing names already carry it, and the guides drifted from what the repo actually did.
- **It does not clean up its own diff.** Writing the minimum and letting `cr` judge it is the point. Reviewing your own work in the context that produced it misses exactly what a separate review catches.
- **Code review runs in a sub-agent.** The review reads a lot; the report is short. Only the report comes back.
- **The review is reported by what's left to decide.** A finding that still stands — not applied, whatever the reason — comes back verbatim with the reason it stands. Findings it applied, and ones the diff already satisfied, are counted in the opening line and never listed: the diff already says what changed, and a roll-call of fixed things is noise nobody reads.
- **Every axis gets a heading and a line**, whether nothing stood on it or it never ran. A skipped axis that looks like a clean one is how a review that missed a whole check passes for a pass.
- **The briefing beats the review.** A finding that contradicts a decision the briefing states is dismissed with the decision quoted. `cr` reads the briefing too, but a settled decision outranks a reviewer's read of it.
- **Integration over unit.** Tests go through the real interface with the real code behind it. Unit tests are reserved for logic worth stressing on its own — complex parsing, non-trivial calculation, boundaries a caller can't reach from outside. Repo conventions come first where they exist. Same rule [issues](../issues/) uses when it names test cases.
- **A test that passes the moment it's written is treated as broken.** No red means it proves nothing.
- **Work with nothing runnable skips the test** — markdown, config, CI. It says so in one line rather than faking an assertion. The escape doesn't apply when a runner exists and the change touches code it executes.
- **It commits nothing and stages nothing** — not even `git add -N`. The tree is left dirty and the files are named, so the user reviews before anything lands.
- **The briefing is read-only.** No ticked boxes, no appended log. Nothing on disk records that it was implemented, so a second run redoes it.
