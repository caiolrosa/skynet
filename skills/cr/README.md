# Cr skill

Reviews a diff along three axes in parallel sub-agents and reports the findings. It never touches the code.

Fifth in the chain: [probe](../probe/) settles the decisions, [spec](../spec/) writes them up, [issues](../issues/) slices them into briefings, [impl](../impl/) builds one, `cr` reviews what it built. A human can also run it on a branch or a dirty tree with no issue in sight.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/cr ~/.claude/skills/cr
```

## How it works

`/cr` reviews the uncommitted working tree. `/cr <branch_name>` reviews everything committed since that branch plus the uncommitted tree, and a commit SHA, a tag or `HEAD~5` works the same way. `/cr issues/03-thing.md` adds the issue as the spec source. Order of arguments doesn't matter — a path that exists is the issue, anything else is a fixed point.

Before spawning anything it pins the diff, confirms the ref resolves, picks up untracked files that `git diff` misses, drops lockfiles and generated code, and finds the repo's standards docs. A bad ref or an empty diff stops it there.

Then three sub-agents in parallel, all three holding the issue file:

- **Standards** — the repo's documented standards, the smell baseline, `readability.md` for prose, and the test rules. Findings marked **hard** or **judgement**.
- **Spec** — missing requirements, scope creep, requirements implemented wrong, each quoting the issue line. Same markings.
- **Correctness** — unhandled edges and hidden bugs, each labelled **certain**, **likely** or **speculative**.

The report is three headings, verbatim, with a count per axis. An axis that skipped or failed says so in place of its findings.

## Design decisions

- **Three axes, never merged and never ranked against each other.** Code can follow every standard and implement the wrong thing, or do exactly what the issue asked and break every convention. Reporting them in one list lets a clean axis mask a failing one.
- **The uncommitted tree is the default.** `impl` never commits and never stages, so after a run `HEAD` is unchanged. A review that asks for a fixed point first would review an empty diff.
- **Untracked files are read by hand.** `git diff` doesn't show them and `impl` doesn't stage, so new files would otherwise be invisible to every axis.
- **Correctness is a third axis, not folded into Standards.** Bug-hunting and convention-checking look for different things, and an agent asked for both does neither well.
- **Different markings per axis.** "A documented standard is broken" is a fact, so Standards and Spec mark hard vs judgement. A bug found by reading is a claim about what would happen, so Correctness marks certain, likely or speculative. One vocabulary across all three would flatten the difference.
- **No severity levels.** Blocker/major/minor invites ranking across axes, which is the exact collapse the split exists to stop.
- **No cap on findings.** The bound is one or two lines per finding, not a word budget per report. A cap trades coverage for a uniform report size, and a truncated review reads exactly like a clean one.
- **Test quality lives on the Standards axis.** Tests are part of the diff and `impl` can't catch its own violations — a test that mocks the project's own code, or that would have passed before the change, ships otherwise.
- **All three agents read the issue.** An agent that hasn't flags work the issue mandated as a hasty abstraction or scope creep, and the caller dismisses it by hand every run.
- **The smell baseline is pasted in full, and the repo overrides it.** It gives the Standards axis something to review against in a repo that documents nothing, without outranking a repo that documents something.
- **No spec source means no Spec axis.** Substituting a branch name or a commit message produces findings against requirements nobody agreed to — the thing `issues` and `impl` both refuse to do.
- **A skipped or failed axis is reported, never omitted.** Two sections where three were promised is how a change gets trusted for a check that never ran.
- **It doesn't run the test suite.** `impl` already did, and a reviewer that runs tests starts fixing them.
- **It never fixes anything.** The caller applies findings. Reviewing and fixing in the same context is what separating the two prevents.
