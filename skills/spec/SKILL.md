---
name: spec
description: "Generate a technical specification document optimized for implementation by AI agents. Use when starting a new feature, planning a complex change, or when asked to create a spec. Triggers on: create a spec, write spec for, specification for, spec out, technical spec, implementation spec."
user-invocable: true
argument-hint: "[feature description]"
---

# Specification Document Generator

Generate technical specification documents that are optimized for implementation by AI agents, with careful attention to context window efficiency and completeness.

---

## Core Principles

1. **NEVER GUESS.** If anything is unclear, ambiguous, or has multiple valid interpretations — ASK. Do not assume, infer, or fill in gaps with your own judgment.
2. **Context window awareness.** The spec must be structured so an implementing agent can work on one section at a time without needing the entire document in context.
3. **Edge case prevention.** Systematically identify and document edge cases, error states, and boundary conditions during the questioning phase.
4. **Implementation-ready.** Every section must be specific enough that an agent can implement it without follow-up questions.

---

## Wizard Flow

This skill operates as a step-by-step wizard. Follow each phase in order. Do NOT skip or combine phases.

---

### Phase 1: Gather the Idea

If `$ARGUMENTS` is non-empty, use it as the initial description and skip the prompt below. Acknowledge what they described and move directly to Phase 2.

If `$ARGUMENTS` is empty, ask the user:

*"What would you like to spec out? Describe the feature, change, or system you have in mind. Include as much or as little detail as you'd like — I'll ask follow-up questions to fill in the gaps."*

Wait for the user's response. Do NOT proceed until you have their initial description.

---

### Phase 2: Codebase Scan

Before asking the user questions, explore the existing codebase to answer what you can on your own. Look for:

- **Frameworks & libraries** in use (check `package.json`, `requirements.txt`, `go.mod`, `Cargo.toml`, etc.)
- **Existing patterns** — how similar features are structured, naming conventions, directory layout. Identify 1-3 specific files that best represent the patterns the new feature should follow.
- **Related code** — modules, components, or services that the new feature will interact with
- **Configuration** — environment variables, config files, database schemas already in place
- **Test setup** — how tests are organized, what test frameworks are used
- **Related documents** — existing specs, PRDs, or similar features that serve as prior art

If no codebase is detected (empty directory, no source files, no dependency manifests), note this is a greenfield project and skip to Phase 3.

Summarize your findings to the user before moving to Phase 3. Include a **Patterns to follow** subsection listing the specific files you identified as reference implementations, with a one-line note on what each demonstrates. These feed into the spec's Related Documents section as prior art. This step prevents asking questions the codebase already answers and gives the user a chance to correct misunderstandings.

---

### Phase 3: Clarifying Questions (MANDATORY)

Now analyze the user's description alongside your codebase findings and identify every remaining gap in understanding.

**Use `AskUserQuestion`** to ask questions interactively, in batches of up to 4 questions at a time. When clear options exist, provide them as choices. When a question is open-ended, provide your best guesses as options (the user can always select "Other"). Use `multiSelect: true` when multiple options can apply simultaneously. Use short `header` labels (max 12 chars) to categorize each question (e.g., "Scope", "Data", "Security").

#### Question Categories (ask about ALL that are relevant):

**A. Scope & Boundaries**
- What is included vs excluded?
- What is the minimum viable version vs nice-to-have?
- Are there existing systems this interacts with?

**B. Behavior & Logic**
- What happens in the happy path?
- What are the inputs, outputs, and transformations?
- What are the exact business rules or logic conditions?

**C. Data & State**
- What data structures are involved?
- Where does state live (database, memory, URL, local storage)?
- What are the data validation rules?
- Are there migration concerns?

**D. Environment & Constraints**
- Are there performance requirements (response time, payload size)?
- Are there accessibility requirements?
- What browsers/platforms must be supported?

**E. Security**
- What are the authentication and authorization requirements?
- What inputs come from untrusted sources and need validation?
- Are there abuse prevention requirements (e.g., rate limiting, replay protection)?
- Is there sensitive data that needs encryption at rest or in transit?
- How are secrets (API keys, credentials, tokens) managed?
- Are there data privacy or compliance requirements (GDPR, HIPAA, etc.)?
- What should be logged vs. redacted in logs?

**F. Dependencies & Integration**
- What APIs, services, or modules does this depend on?
- What existing code should be reused vs written fresh?

**G. General / Uncategorized**
- Anything that doesn't fit the categories above but is relevant to building a complete understanding
- User preferences, conventions, past decisions, or tribal knowledge that could affect implementation
- Anything the agent notices is missing, feels off, or would benefit from the user's direct input
- If something seems implied but was never stated explicitly — ask here rather than assume

#### Rules for Questioning:

- **Ask at least a few questions** in the first round, covering at minimum: scope and happy path behavior. Scale the number of questions to the complexity of the feature.
- **Do NOT proceed to the next phase until the user confirms they have no more context to add**
- After each round of answers, evaluate: "Do I still have any assumptions?" If yes, ask another round
- At the end of questioning, use `AskUserQuestion` to ask: *"Before I move on, is there anything else I should know? Any constraints, preferences, or context I'm missing?"* with options like "No, I'm good — move on" / "Yes, I have more context".
- If the user's answers reveal new ambiguity, ask follow-up questions — do not silently resolve them
- **Maximum 3 rounds of questions.** After 3 rounds, capture any remaining unknowns in the spec's "Open Questions" section rather than continuing to ask. Respect the user's time.

---

### Phase 4: Edge Case Brainstorm (MANDATORY)

Now that the requirements are understood, systematically identify what could go wrong. Consider:

- Empty, null, or undefined inputs
- External dependency failures
- Boundary values (min, max, overflow)
- Concurrent access or race conditions
- User cancellation mid-operation
- Timeout behaviors

Identify **up to 10** of the most critical edge cases, scaled to the feature's complexity (a simple feature may only warrant 3–5). If you've identified more, mention the count and offer to expand the list on request.

First, present the edge cases as a numbered list (scenario + your assumed behavior) so the user can see them all at once. Then use `AskUserQuestion` to confirm them in batches:

- Use `multiSelect: true` with a question like *"Which of these edge case behaviors look correct?"*
- List each edge case as an option with the scenario as the label and your assumed behavior as the description
- Process up to 4 edge cases per `AskUserQuestion` call; use multiple calls for larger lists
- For any edge case the user did NOT select (i.e., they disagree with your assumed behavior), ask a follow-up question to clarify the correct behavior

After all edge cases are confirmed, use `AskUserQuestion` to ask: *"Are there edge cases I'm missing?"* with options like "No, looks complete" / "Yes, I have more".

If the user's confirmations or corrections reveal new ambiguities, return to Phase 3 for a follow-up round of questions (this does NOT count toward the Phase 3 maximum of 3 rounds). Then come back to Phase 4 to re-confirm any affected edge cases before proceeding.

Wait for confirmation before proceeding.

---

### Phase 5: Choose Output Location

Now that the feature is fully understood, suggest a default path based on the feature name and ask the user to confirm or override. Use `AskUserQuestion` with options like:

- `specs/spec-<feature-slug>.md` (label: "Default path", description showing the full path)
- "Different path" (let the user type a custom path via "Other")

Use whatever path the user confirms. Do NOT proceed until you have a confirmed path.

---

### Phase 6: Generate the Spec

Only after all previous phases are complete, load the spec template from [template.md](template.md) and generate the spec. Refer to [example.md](example.md) for the expected tone, format, and level of detail.

**Important:** Only include sections that are relevant to this specific feature. Omit sections entirely if they don't apply (e.g. skip "UI/UX Specification" for backend-only work, skip "API Contracts" if no APIs are involved). Do NOT write "N/A" stubs — just leave the section out.

Do NOT save yet — proceed to Phase 7.

---

### Phase 7: Validate Before Saving

Re-read the generated spec and check for the following issues:

1. **Weasel words** — scan for "should", "probably", "maybe", "as appropriate", "might", "could consider" and replace with precise language
2. **Missing behavior** — every REQ must have a `Behavior:` block
3. **Broken dependency ordering** — no REQ should `Depend On` a later REQ
4. **Stub sections** — remove any section that's just "N/A" or a single sentence
5. **Implementation steps** — every step must have `Done when:` with machine-verifiable criteria, and file lists when paths are known (skip file lists for greenfield)
6. **Missing or weak Agent Boundaries** — if the feature touches existing code, shared services, or has security implications, include an Agent Boundaries section with concrete Always/Ask first/Never items

After fixing all issues, show the user a brief summary of what was caught and corrected. For example:

*"Validation pass complete. I fixed 3 issues before saving: replaced 2 weasel words ('should' → precise language) and removed 1 stub section (Testing Strategy had no content). Everything else checked out."*

If no issues were found, say so: *"Validation pass complete — no issues found."*

#### Size check

After validation, review the spec's scope with a focus on the number of REQs and whether they form natural independent workstreams. If the spec feels too large for an implementing agent to tackle in a single focused effort — e.g., it covers many REQs spanning loosely related concerns — propose splitting it:

*"This spec covers [N] requirements across [describe the distinct areas]. I think it would work better as [2–3] separate specs:*

*1. **[Spec A title]** — [which REQs / scope]*
*2. **[Spec B title]** — [which REQs / scope]*

*Each would reference the others where they share dependencies. Does this split make sense, or would you draw the line differently?"*

If the user agrees to split, generate each spec separately (repeating Phase 6 and 7 for each), saving them to paths confirmed with the user. Each spec should include a "Related Specs" note at the top of its Overview section linking to its siblings.

If the user prefers to keep it as one spec, respect that and proceed to save.

If the spec is tightly focused and cohesive, skip this step and save directly to the path confirmed in Phase 5.

---

## Writing Guidelines

- **Be precise:** "The button is disabled when the form has validation errors" NOT "The button should probably be disabled when appropriate"
- **Use concrete values:** "Max 255 characters" NOT "reasonable length limit"
- **Name things:** Use exact function names, component names, CSS classes, route paths where known
- **Show, don't tell:** Include code snippets for complex logic, regex patterns, data shapes
- **One interpretation:** If a sentence could be read two ways, rewrite it until it can't
- **Cross-reference:** Requirements should reference each other by ID (REQ-001, REQ-002) to show dependencies
- **Self-contained REQs:** Each requirement must include all information needed to implement it without reading other sections
- **No buried context:** Critical information (env vars, config values, API keys) must be in a dedicated section, not scattered throughout
