---
name: impl
description: "Build the work in a briefing, one test at a time, then hand it to code review. Use after issues, or whenever a piece of work is decided and ready to build. Triggers on: implement this issue, build this issue, work on issue 03, implement it, build this ticket."
user-invocable: true
argument-hint: "[path to a briefing]"
---

# Impl

Build the work described in a briefing — an issue file, a spec, a paragraph in chat. One test, then the code that passes it, then the next test. When it's done, hand it to `cr`.

You build. You do not decide what to build — the briefing settled that. You do not commit. See [Never commit](#never-commit).

---

## Core rules

1. **The briefing is the source of truth,** whatever shape it arrived in. Don't re-open decisions it made. Don't add what it didn't ask for.
2. **Test first.** One failing test, then the smallest code that passes it. Never the other way round.
3. **The repo's standards beat your habits.** Every time.
4. **Write the minimum,** and don't clean up as you go.
5. **You don't review your own diff.** `cr` does, and you apply what it finds.
6. **Never commit and never stage.** The user reviews the tree and decides.

---

## Reading the briefing

A path: read it. No path: the work described earlier in the session. Nothing anywhere: ask what to build.

**No required headings, ever.** Read what you were handed for three things, wherever they sit and whatever they're called:

- **The outcomes** — what's true once this is done.
- **The cases** — what to test, and the input or boundary that makes each one bite.
- **The constraints** — decisions and conventions the work has to honour.

A briefing that states none of them still gets built. Derive what's missing from what's there, say in one line what you derived, and carry on.

**Derive, never invent.** Reading *"replaying a delivered row is rejected"* out of a stated rule is deriving. Adding pagination nobody mentioned is scope.

---

## Standards

Four things before you write, in this order:

1. **The repo's standards docs.** `CLAUDE.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CODING_STANDARDS.md` — whatever documents how code here is written.
2. **Nearby code of the same kind.** Whatever the briefing says to match tells you where to look; the code tells you how it's actually done — layout, naming, how tests are written and where they live.
3. **How this repo verifies.** The CI config first — it already says which commands run for which paths. Then Makefiles and package scripts near what you're changing. **If you can't find them, ask.** A guessed test command that passes because it ran nothing is worse than no test command.
4. **The working tree.** Run `git status`. If it's already dirty, keep the list — those files are not yours, you don't touch them, and you name them separately at the end.

Don't survey the whole codebase. Read what the work touches.

---

## The loop

A slice is one test and the code that makes it pass.

1. Write one test. Run it. **It must fail.**
2. Write the smallest code that makes it pass. Nothing more.
3. Run that test file, plus typecheck or compile.
4. Next slice.

Once at the end: the full suite, plus lint.

Rules that bind every slice:

- **Red before green.** A test that passes the moment you write it proves nothing. Treat it as broken and fix the test before you write any code.
- **One test at a time.** Tests written in bulk describe behaviour you imagined rather than behaviour you built, and they lock in a structure you chose before understanding the work.
- **The minimum to pass.** No speculative options, no hooks for a future nobody asked for.
- **No cleanup.** Don't refactor or rename as you go. Duplication included — leave the copies alone, because the wrong abstraction costs more than the copy.
- **Five failed attempts at the same test means stop and ask.** The next move is usually weakening or deleting the test. Don't.
- **A final suite failure in code you didn't touch means stop and say so.** Pre-existing breakage is not yours to fix or to hide, and the work isn't done over a red suite.

---

## Testing

Take the cases the briefing names, with whatever input or boundary it gives for each. Where it names none, derive them from the outcomes it states, say in one line what you derived, and carry on.

Follow the repo's own testing conventions first — its framework, its layout, its style. Where it has none:

- **Integration over unit.** Test through the real interface a caller uses, with the real code behind it. A pile of unit tests that replace everything the code calls with a fake only tells you the fakes agree with each other.
- **Unit tests only for logic worth stressing on its own** — complex parsing, a non-trivial calculation, boundaries a caller can't easily reach from outside.

A good test:

- **Goes through the public interface.** The code behind it can change completely and the test still passes.
- **Names behaviour, not mechanics.** *"user can check out with a valid cart"*, not *"checkout calls processPayment"*.
- **Takes its expected value from somewhere independent** — a known-good literal, a worked example, the briefing. Not a value the test works out by repeating the logic the code uses.
- **Keeps assertions that belong together in the same test.** Checking six fields on one response is one test, not six. A second, unrelated behaviour is what doesn't belong.

Three ways a test goes bad:

- **Tied to the implementation.** It fakes out your own code, reaches into private functions, or checks the result through a side channel — reading the database directly instead of calling the function that reads it. The tell: it breaks when you reorganise code that behaves exactly the same.
- **Expecting a value the test computed the same way the code does.** Both copies share the same mistake, so the test passes anyway.
- **Written in bulk, before any code.** It describes behaviour you imagined, and it fixes the shape of the work before you understand it.

### Mocking

Mock at system boundaries only: external APIs, time and randomness, databases and the file system when running the real thing isn't practical — and prefer real files in a temp directory where you can.

**Don't mock your own code.** If something you wrote is hard to test without replacing it with a fake, the interface is the problem.

### When there is nothing to run

Some work produces no runnable behaviour — markdown, a config file, a CI workflow. Say so in one line, skip the test, and do the work.

This applies only when nothing runnable exists. If the repo has a test runner and your change touches code that runner executes, write the test.

---

## Review

When the work is done and the suite is green, hand it to `cr`.

**Send it to a sub-agent and wait.** Pass the path to the briefing, when it is a file. `cr` works out its own diff — you don't build one or describe your changes. The sub-agent keeps the review's reading out of this context.

**What comes back is a report**: three axes — Standards, Spec, Correctness — each finding one or two lines carrying the file, the claim, the suggested fix, and a marking.

Then:

1. Apply the findings you agree with.
2. Re-run the full suite and lint.
3. Report what's left under `cr`'s axis headings, keeping its `Reviewed …` and `Excluded: …` lines above them as written — the excluded files are part of what the review didn't cover.

**Only what you didn't apply reaches the user.** Every finding lands in one of three buckets:

- **It stands.** You didn't apply it, whatever the reason — you disagree, the briefing overrode it, or you applied only part. Reproduce it verbatim, `cr`'s file and marking and claim, and add your reason. Where the briefing overrode it, the quoted decision is that reason; where you applied part, what's left undone is. These are the only findings the user has to decide on, so nothing about them is shortened, reworded or moved off its own line.
- **You applied it.** Counted, never listed. The diff already says what changed.
- **It needed no action.** The diff already satisfied it, so `cr` read the code wrong. Counted, never listed, so a misread doesn't pass for a clean bill.

**Open with one count across all three axes**: `2 stand · 7 applied · 1 already satisfied`. Drop any term that's zero. No total.

**Every axis gets its heading and at least one line.** Where nothing stands, that line is `Nothing stands.`; where the axis didn't run, it's `cr`'s reason. A clean axis and a skipped one must never look alike.

**The briefing beats the review.** When a finding contradicts a decision the briefing states, dismiss it and quote the decision. `cr` reads the briefing too, but a decision the team settled outranks a reviewer's read of it.

**If `cr` isn't installed, or the sub-agent comes back with nothing**, say the review didn't run and why. Don't review the diff yourself instead — a review by the agent that wrote the code is the thing `cr` exists to replace.

---

## Never commit

No `git commit`, no `git add`, no `git add -N`, no branch. The tree is left dirty and you name the files you changed, plus any that were already dirty before you started — that list is how the user finds the work.

The briefing is read-only. Don't tick items in it, don't append a log to it, don't move it. Nothing on disk records that it was implemented, so re-running `/impl` on the same briefing does the work again.
