---
name: plan
description: "Explore and create a plan for implementation. Triggers on: write the plan, write the spec, plan this out, spec this out, turn this into a spec, write this up, plan this out"
user-invocable: true
argument-hint: "[initial context]"
---

# Plan
You will work in two phases:

- Phase 1 is the exploration phase, here you will research the
codebase if related to code, and the internet or any other context provided by the user for 
non related work.
- Phase 2 is the planning phase, here you will write one section at a time, the user approves
each before you writing the next.

You explore and write the plan. Never implement.

## Rules
- Use [skills/readability.md](../readability.md) as the wording guideline.
- Use AskUserQuestion to ask questions to the user, unless instructed against it.
- When in doubt, ask.
- Don't assume, ask.
- Whenever there is contradiction, either you found something or something the user said
contradicts, point it out to the user, ask him, don't assume an answer.
- When there are multiple interpretations, ask.

## Phase 1: Exploration
- Read the user's input, if none was provided ask.
- Explore the codebase and/or any artifact provided by the user.
- Ask questions until a shared understanding between you and the user is found.
- Use a tree shape model to ensure you reach shared understanding.
- Look for any relevant error or edge cases and work through it with the user. 

## Phase 2: Planning
- Ask the user where you should write the plan.md file.
- Read [example.md](./example.md) this is the structure the document should follow.
- You should work with the user section by section, write the section first,
ask the user to review it, wait for approval and move on to the next.

### Context section
Explains what and why we are planning, keep it short nobody 
wants to read a wall of text. Prefer bullet points over prose.

### Out of scope
Contains everything the user mentioned as being out of scope 
or won't be planned at this point. Same as context keep it short.

### Flow section
This section is dependent on the type of work, if the work 
being planned does not benefit from a flow diagram, sequence 
diagram, ERDs, state diagrams, flow chart, etc, don't add it.
Diagrams should stand on their own, no added prose to explain them.

### Prototype
- This section is dependent on the type of work, this is
usually beneficial for UI, document or just exploration work,
ask the user if he wants the prototype.
- Prototypes should stand on their own, no added prose to
explain them.
- Be as faithful to the repo's design as possible, use it's 
design tokens, colors etc. If none available keep it unstyled 
and tell the user so.
- Prototypes are static fixtures, don't call APIs, mock them 
the goal is to see how it would look like, not make it work.

### Plan sections
Everything that will be built by the plan should be 
sections of work that are testable input to output (end to end)
each section will have the following blocks, but not limited to.

| Block | Content |
|---|---|
| Prototype | A link to the prototype file related to this section |
| Schema | Usually related to databse tables and columns, as a migrtation or DDL in the repo's idiom |
| Inputs & Outputs | Any kind of IO the application has, HTTP and it's details, RPC, Websocket, File, etc. |
| Types | The types this section defines |
| Interfaces | Signatures and what they return |
| Callstack | The call tree from input to output |
| Constraints | What must hold - validations, guarantees, invariants |

Only add the blocks that apply, a migration section has only a schema, a UI section won't have a schema but may have a prototype for that specific section.

The blocks above are not a closed list, they are the most common ones, you may add anything else that applies to the work being planned, each section defines the contracts and interfaces it abides by.

If any correction the user makes contradicts something else 
in the document, ask the user about it.

### Open questions section
Anything that was left unanswered ideally this should be empty 
but it's possible the user doesn't have all the answers yet 
those should be here.

### QA Plan
A bullet list of everything the user should manually QA after 
the work is complete to validate everything works as expected.

## Wrap up
Go through the entire finished document, read it again and fix what you find:

- An entry that depends on a later one or points forward.
- A type defined in two entries, or referenced nowhere.
- A constraint restating what the block above it already shows.
- Two entries stating the same guarantee differently.
- Bullets or prose doing a job an artifact would do better.
- An empty section, or one padded to look full.
- A boundary in a Flow diagram that no callstack or interface mentions.

Fix them silently, then say what you fixed in one line: 
*"Fixed 2 things before saving: a vague constraint in entry 4, and entry 3 pointed forward to entry 4's limit."* Say so when there was nothing.
