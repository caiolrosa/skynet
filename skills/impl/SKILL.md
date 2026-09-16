---
name: impl
description: "Build the work in one issue file, one test at a time, then hand it to code review. Use after issues, when a briefing is ready to build. Triggers on: implement this issue, build this issue, work on issue 03, implement it, build this ticket."
user-invocable: true
argument-hint: "[path to issue file]"
---

# Impl

Build the work described in one issue file. One test, then the code that passes it, then the next test. When the work is done, hand it to `cr`.

You build. You do not decide what to build — the issue settled that. You do not clean up your own diff, and you do not commit. See [Never commit](#never-commit).

---

## Core rules

1. **The issue is the source of truth.** Don't re-open decisions it already made. Don't add what it didn't ask for.
2. **Test first.** One failing test, then the smallest code that passes it. Never the other way round.
3. **Ask only when blocked.** Four things stop the run — see [Stopping](#stopping). Everything else you decide, do, and report.
4. **Write the minimum.** Cleaning up is `cr`'s job, not yours.
5. **Never commit and never stage.** The user reviews the tree and decides.

---

## Input

| Given | Use |
|---|---|
| `/impl issues/03-thing.md` | That issue. |
| `/impl`, work described earlier in the session | That work, with the note below. |
| `/impl path/to/spec.md` | The first work item in it, with the note below. |
| `/impl`, nothing anywhere | Ask what to build. |

### When it isn't an issue

An issue file carries `Done when`, `Tests`, and the decisions that bind the work. A spec or a plain sentence carries none of that.

Say so once — *"this isn't an issue file, so there's no `Done when` or `Tests`; working from what's here"* — and then run normally. Don't invent acceptance criteria to fill the gap. That is what `probe`, `spec` and `issues` are for, and inventing them here produces a report that claims outcomes nobody agreed to.

---

## Before you write

Four things, in this order:

1. **The repo's standards.** `CLAUDE.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CODING_STANDARDS.md` — whatever documents how code here is written. `cr` reads the same files afterwards, so anything you skip comes back as a finding.
2. **Nearby code of the same kind.** The issue's `Conventions to follow` says what to match; read the code to learn how it's actually done — layout, naming, how tests are written and where they live.
3. **How this repo verifies.** See [Verifying](#verifying).
4. **The working tree.** Run `git status`. If it is already dirty, say so in one line and keep the list. Those files are not yours, you don't touch them, and the final report names them separately.

Don't survey the whole codebase. Read what the issue touches.

---

## The loop

One slice at a time. A slice is one test and the code that makes it pass.

1. Write one test. Run it. **It must fail.**
2. Write the smallest code that makes it pass. Nothing more.
3. Run the fast check (see [Verifying](#verifying)).
4. Next slice.

Rules that bind every slice:

- **Red before green.** A test that passes the moment you write it proves nothing. Treat it as broken and fix the test before you write any code.
- **One test at a time.** Never write all the tests up front. Tests written in bulk describe behaviour you imagined rather than behaviour you built, they stop reacting to what the last slice taught you, and they lock in a structure you chose before understanding the work.
- **The minimum to pass.** No speculative options, no hooks for a future the issue didn't ask for.
- **No cleanup.** Don't refactor or rename as you go. `cr` reviews the diff and that is where cleanup is decided. Duplication is the exception: leave the copies alone. `cr` won't ask you to extract them, and the wrong abstraction costs more than the copy.

### What to test

Take the cases from the issue's `Tests` section.

If the issue has no `Tests` section, derive the cases from `Done when`, state them in one line, and carry on.

### What a good test looks like

**Integration over unit.** Test through the real interface a caller uses, with the real code behind it. A test that exercises the actual path is the one that tells you the feature works; a pile of unit tests that replace everything the code calls with a fake only tells you the fakes agree with each other.

**Unit tests only for logic worth stressing on its own** — complex parsing, a non-trivial calculation, boundaries and error paths a caller can't easily reach from outside. Isolating that logic lets you hit it hard. Everything else is covered better from the outside.

Follow the repo's own testing conventions first. Where it has none, this is the default.

Either way, a good test:

- **Goes through the public interface.** The code behind it can change completely and the test should still pass.
- **Names behaviour, not mechanics.** *"user can check out with a valid cart"*, not *"checkout calls processPayment"*.
- **Takes its expected value from somewhere independent** — a known-good literal, a worked example, the issue. Not a value the test works out by repeating the logic the code uses.
- **Keeps assertions that belong together in the same test.** One behaviour per test, with as many assertions as that behaviour needs — checking six fields on one response is one test, not six. What doesn't belong is a second, unrelated behaviour.

Three ways a test goes bad:

- **Tied to the implementation.** It fakes out your own code, reaches into private functions, or checks the result through a side channel — reading the database directly instead of calling the function that reads it. The tell: it breaks when you reorganise code that behaves exactly the same.
- **Expecting a value the test computed the same way the code does.** Both copies share the same mistake, so the test passes anyway. Use a known-good literal.
- **Written in bulk, before any code.** It describes behaviour you imagined rather than behaviour you built, and it fixes the shape of the work before you understand it.

### Mocking

Mock at system boundaries only:

- External APIs
- Databases, only when the project's tests have no real one available or running one isn't practical
- The file system, on the same terms — use real files in a temp directory where you can
- Time and randomness

Don't mock your own code — not the modules it calls, not anything else you control. If something you wrote is hard to test without replacing it with a fake, the interface is the problem.

### When there is nothing to run

Some work produces no runnable behaviour — markdown, a config file, a CI workflow. Say so in one line, skip the test, and do the work.

This applies only when nothing runnable exists. If the repo has a test runner and your change touches code that runner executes, write the test.

---

## Verifying

**Find the commands before you start.** Look at the CI config first — it already says which commands run for which paths. Then Makefiles and package scripts near the files you're changing.

**If you can't find them, or you're not sure which apply, ask.** A guessed test command that passes because it ran nothing is worse than no test command.

| When | Run |
|---|---|
| Every slice | The single test file, plus typecheck or compile |
| Once at the end | The full suite, plus lint |

If the final suite fails on something you didn't touch, stop and report it. Don't fix unrelated code, and don't call the work done over a red suite.

---

## Review

When the work is done and verified, hand it to `cr`.

**Send it to a sub-agent and wait.** Pass the path to the issue file. `cr` works out its own diff — you don't build one or describe your changes. The sub-agent keeps the review's reading out of this context.

**What comes back is a report**, nothing else: three axes — Standards, Spec, Correctness — each finding one or two lines carrying the file, the claim, the suggested fix, and a marking. Standards and Spec mark **hard** or **judgement**; Correctness marks **certain**, **likely** or **speculative**.

Then:

1. Apply the findings you agree with.
2. Re-run the full suite and lint.
3. **Reproduce the report in yours, verbatim**, with every finding marked ✅ fixed or ❌ dismissed and the reason. Demote its axis headings to `###` and change nothing else. The user reads the review, not your summary of it.

**The issue beats the review.** When a finding contradicts something under `Decisions that bind this`, dismiss it and quote the decision. `cr` reads the issue too, but a decision the team settled outranks a reviewer's read of it.

**If `cr` isn't installed, or the sub-agent comes back with nothing**, say the review didn't run and why. Don't review the diff yourself instead — a review by the agent that wrote the code is the thing `cr` exists to replace.

---

## Stopping

Four things stop the run and ask the user:

| Stop | Why |
|---|---|
| The repo contradicts a decision that binds the issue | Carrying on means shipping against a decision the team made. |
| Five failed attempts at the same test | The next move is usually weakening or deleting the test. Don't. |
| The final suite fails on something you didn't touch | Pre-existing breakage is not yours to fix or to hide. |
| You can't work out how the repo verifies | See [Verifying](#verifying). |

Everything else: decide it, do it, and put it in the report.

---

## The report

```markdown
## What I built
<a sentence or two>

## Slices
- <test name> — <what it covers>

## Done when
- ✅ <item>
- ❌ <item> — <why not>

## Verify
- `<command>` — <result>

## Review
### Standards
- ✅ <finding> — <what changed>
- ❌ <finding> — <why it stands>
### Spec
- ✅ <finding> — <what changed>
### Correctness
- ❌ <finding> — <why it stands>

<cr's count line>

## Files changed
- <path>

Already dirty before this run, untouched:
- <path>
```

Omit a section that has nothing in it. Say plainly when something failed — a report that reads clean over a red suite is worse than no report.

Nothing is committed, so the last two sections are how the user finds the work.

---

## Never commit

No `git commit`, no `git add`, no `git add -N`, no branch. The tree is left dirty and the report names the files.

The issue file is read-only. Don't tick its `Done when` items, don't append a log to it, don't move it. Nothing on disk records that an issue was implemented — that's deliberate, and it means re-running `/impl` on the same issue does the work again.
