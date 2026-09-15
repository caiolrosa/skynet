# Probe skill

A skill that interviews you about a plan, design, or decision until every branch is settled — then hands back a shared understanding and stops.

## Files

- **SKILL.md** — Skill definition.

## Installation

```sh
cp -r skills/probe ~/.claude/skills/probe
```

## How it works

Your decision is modelled as a **tree**. A node is one decision; a node hangs off another when it only makes sense once its parent is settled. Nodes are settled (`✅`), open (`❓`) or blocked (`🔒`).

Each round:

1. The tree is printed, settled branches collapsed to one line.
2. Up to 4 open questions are asked through `AskUserQuestion`, highest-impact first. The rest roll into the next round.
3. Your answers reshape the tree — new branches appear, blocked ones unblock.

Before wrapping up, it sweeps for edge cases and confirms them with you, then writes a short summary of what was decided, what was rejected and why, what's still assumed, and what's left open. It offers to save that to a file.

## Design decisions

- **The tree is on screen.** Printing it means you can correct it.
- **Assumptions are visible every round.** Including anything the agent decided on your behalf when you said "you decide".
- **Blocked questions are never asked.** A question whose premise isn't settled either forces you to answer a hypothetical or make an assumption that becomes load-bearing.
- **Facts are the agent's job.** It dispatches subagents for anything in the environment rather than asking you, and only the questions downstream of a lookup wait for it.
- **It pushes back.** Vague or self-contradictory answers get named and re-asked until resolved — unless you say "take it as-is", which stops it immediately and logs the tension instead.
- **The edge-case sweep has a relevance bar, not a quota.** It raises as many or as few cases as the subject warrants, but only ones where getting it wrong changes a decision or costs real work — minor cases you'd wave through are dropped.
- **You can stop at any time.** There's no round cap; if answers stop reshaping the tree the agent says so once and recommends wrapping up.
- **It never implements.** Output is understanding, plus a file if you ask for one.
