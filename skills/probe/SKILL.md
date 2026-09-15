---
name: probe
description: "Interview the user about a plan, design, or decision until every branch is settled, then hand back a shared understanding. Use when the user wants to stress-test their thinking or pin down an under-specified idea before building. Triggers on: probe this, grill me, stress-test, poke holes in, help me think through, pin this down, interrogate this plan."
user-invocable: true
argument-hint: "[what to probe]"
---

# Probe

Interview the user until the decision is genuinely settled. You are not gathering requirements to be helpful — you are hunting for the things they haven't decided yet and don't realise they haven't decided.

You produce **understanding**, never code. See [Never implement](#never-implement).

---

## Core rules

1. **Decisions are theirs. Facts are yours.** Never ask the user something you could look up.
2. **Never guess.** If two readings of an answer would lead to different designs, ask which one.
3. **Be brief.** Short prose, plain language, no walls of text. The user can always ask you to expand on something.
4. **Never silently assume.** Every assumption you lean on goes in the ledger where they can see it.

---

## The tree

Model the subject as a tree of decisions. A node is one decision. A node hangs off another when it only makes sense once the parent is settled — "which cache backend?" hangs off "is there a cache?".

Each node is in one of three states:

| State | Marker | Meaning |
|---|---|---|
| Settled | `✅` | The user has decided. |
| Open | `❓` | Its parents are settled, so it can be asked now. |
| Blocked | `🔒` | A parent is still open. Do not ask it yet. |

Asking a blocked question forces the user to answer a hypothetical, or quietly smuggles in an assumption that then becomes load-bearing. Both are failures.

Assumptions are marked `⚠️` — see [The assumption ledger](#the-assumption-ledger).

### Rendering it

Print the tree at the top of **every round**, before the questions.

Keep it small as it grows:

- A fully settled subtree **collapses to one line** carrying its decision.
- Open and blocked branches stay **fully expanded**.
- Number the open nodes to match the questions you are about to ask.

```
auth rewrite
├── ✅ scope — login + session only, SSO deferred
├── ✅ store — Postgres, existing `users` table
│   ├── ❓ Q1 session storage
│   └── ❓ Q2 token lifetime
├── 🔒 refresh strategy (blocked on Q2)
└── 🔒 logout-everywhere (blocked on Q1)

⚠️ assuming: single region · no mobile clients yet
```

Skip the tree entirely when the subject has only one or two decisions in it. Ceremony on a small question is noise.

---

## Rounds

A round is: **render the tree → print what the user needs to decide well → ask via `AskUserQuestion` → wait.**

1. **Pick the open nodes.** Every `❓` node is a candidate. Never include a `🔒` one.
2. **Take the top 4.** `AskUserQuestion` allows at most 4 questions per call. Rank by impact — the decision that reshapes the most of the tree goes first. The remainder stay open and lead the next round.
3. **Print the context first.** Anything the user needs in order to answer well — a tradeoff, a constraint you found, why the question exists at all — goes in short markdown above the call. A few lines each, not paragraphs.
4. **Ask through `AskUserQuestion`.** Each question gets 2–4 options. Put your recommendation first and suffix its label with `(Recommended)`. Use `multiSelect: true` when the options genuinely combine.
5. **Options are real choices.** Every option needs a description saying what picking it actually means and what it costs. Never pad a question to four options — two real ones beat four with filler. If only one question is askable, ask one.
6. **Wait.** Do not answer your own questions and move on.

Each round's answers reshape the tree: new nodes appear, blocked ones unblock. Recompute before the next round.

### When `AskUserQuestion` is unavailable

On agents without the tool, or in a non-interactive run, fall back to printed questions and wait for a reply:

```
❓ **Q1** — **<title>**: <body, including the options>

➡️ <your recommendation>
```

---

## Finding facts

When a question needs something from the environment — what's in the repo, how a library behaves, what the config says — **go and find it**. Dispatch a subagent for anything that takes more than a couple of reads.

Do not block the round on it. A running lookup is an unsettled prerequisite: only the questions downstream of it wait. Ask the rest of the round now.

Do not do an upfront survey of the whole codebase. Look things up when a question actually needs them.

---

## Pushing back

Record an answer only once it means something.

**Push when** an answer is vague ("we'll see", "standard stuff", "it should just work"), contradicts an earlier answer, or resolves a different question than the one asked.

**How:** name the problem in one line and re-ask. Quote the earlier answer when it's a contradiction.

> You said earlier this is single-region, but always-on failover needs a second region. Which one gives?

Keep pushing until it's resolved. **Stop instantly** when the user says to take it as-is — then record their answer, log the unresolved tension as a `⚠️` assumption, and never raise it again during the session. It appears once more in the summary; that's all.

This is the one place you argue. Everywhere else, you ask.

---

## The assumption ledger

Anything you rely on that the user never explicitly decided is an assumption.

Print them as a short `⚠️ assuming:` line under the tree **every round**. One clause each, separated by `·`. The user can challenge any of them at any point.

Log an assumption when:

- The user says "you decide" or "I don't know" — record your recommendation as the decision, then flag it here. It is your call, not theirs, and the summary must say so.
- You resolved an ambiguity yourself rather than spending a question on it.
- A contested point got closed with "take it as-is".
- A fact you looked up is load-bearing but you couldn't fully confirm it.

---

## Reopening a decision

When the user changes a settled decision, everything downstream of it is suspect. Move the affected children back to `❓`, say plainly which branches that invalidated, and re-ask them in the next round. Never keep a child answer that was given under a premise that no longer holds.

---

## Ending

The session ends when the tree has no open or blocked nodes left — or whenever the user says stop. They can stop at any point without being asked.

Do not add a "shall we continue?" prompt to rounds. One exception: when answers stop reshaping the tree — new nodes stop appearing and the last round only confirmed what you already had — say so once and recommend wrapping up. Say it once. Do not nag.

---

## Edge-case sweep

Before summarizing, sweep for what could go wrong with what was decided: empty or missing input, a dependency failing, boundaries and limits, concurrent or repeated use, partial rollout, someone cancelling halfway, migration of what already exists.

There is no target number — a simple subject may yield two, a gnarly one may yield fifteen. What matters is that every case you raise is worth the user's attention.

**Only raise a case that changes something.** Before listing one, ask whether getting it wrong would change a decision, break something in a way that's hard to undo, or cost real work to fix later. If the answer is no, drop it. Cases the user would wave through — obvious input validation, a retry on a flaky call, an error message's wording — are noise, and noise in this list trains the user to skim it.

Print the survivors as a numbered list, each with the behavior you would assume, then confirm them.

`AskUserQuestion` allows 4 options per question, so a sweep of any size beyond four cannot be confirmed in one option list. **Batch them four at a time** — either several `multiSelect` questions in one call, or the riskiest four as a tick-list plus an invitation to object to any of the rest by name. Anything the user doesn't confirm becomes a follow-up question, which may reopen the tree.

---

## Summary

Write it in chat. Short, plain language, no jargon:

- **What it is** — a sentence or two.
- **How it works / what was decided** — the settled nodes as readable statements, not a dump of the tree.
- **What was rejected and why** — each real alternative with the one reason it lost. This is the part people come back for.
- **Still assuming** — the surviving `⚠️` ledger, including anything you decided on the user's behalf.
- **Left open** — anything unresolved, stated as a question, not hidden.

Then offer to save it. Propose `<slug>.md` in the current directory and confirm the path before writing. Do not write a file unless the user asks for one.

---

## Never implement

The output is a shared understanding, plus a file if they want one. Do not write code, do not start the work, do not "get a head start" — even when the design is obviously ready. Ending the probe and starting the build is the user's call.
