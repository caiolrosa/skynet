# Spec Template

Use this template when generating the spec in Phase 6. Only include sections that are relevant to the feature — omit any that don't apply. Number sections sequentially in the generated spec based on which sections are actually included — don't leave numbering gaps from omitted sections.

---

> **TL;DR:** [2-4 sentences summarizing what this spec covers, how many requirements it has, roughly how many files are affected, and any notable constraints. This lets an implementing agent quickly decide which sections to load.]
>
> **Done when:** [Machine-verifiable criteria that define when the entire spec is complete — not per-step, but the overall finish line. E.g., "E2E test passes full flow, all endpoints return expected status codes, migration is reversible, no existing tests broken."]

## Overview
- One paragraph: what this is and why it exists
- **Related documents (optional):** Link to related PRDs, sibling specs, or prior art. Include when this feature depends on, extends, or was split from other work.
  - `PRD: path/to/prd.md` — upstream product requirements
  - `Spec: path/to/sibling-spec.md` — sibling or prerequisite spec
  - `Prior art: path/to/file.ts` — existing implementation to reference as a pattern

## Goals & Success Criteria
- Measurable goals (bullet list)
- How do we verify this is done correctly?

## Non-Goals & Out of Scope
- What this spec explicitly does NOT cover
- Adjacent work that might be assumed but is excluded

## Constraints & Assumptions
- Confirmed technical constraints (e.g., "Assumes PostgreSQL 15+")
- Confirmed user/business constraints (e.g., "Must work without JavaScript disabled")
- Assumptions validated during the questioning phase (e.g., "Auth tokens are JWT, stored in httpOnly cookies — existing pattern")

## Agent Boundaries
Guard rails for the implementing agent. Omit this section only if the feature has no meaningful risk of unintended side effects.

- **Always:** [Actions the agent must do without asking — e.g., "Run existing tests after each step", "Use existing auth middleware for new endpoints", "Follow existing naming conventions"]
- **Ask first:** [Actions that require human review before proceeding — e.g., "Adding new dependencies", "Changing database schema beyond what's specified", "Modifying shared utilities used by other features"]
- **Never:** [Hard stops — e.g., "Delete existing data or tables not mentioned in this spec", "Modify files outside the file manifest without explicit approval", "Skip error handling for any endpoint"]

## Architecture & Approach
- High-level approach (which files, modules, patterns)
- Sequence diagram or data flow (ASCII art) if helpful
- Key architectural decisions and WHY they were made
- **File manifest:** List ALL files that will be created or modified so an implementing agent can plan its work up front

## Detailed Requirements

For each unit of work, provide:

```markdown
### REQ-001: [Title]

**What:** [Precise description of what to build/change]

**Where (optional):** [File path(s) — include when the requirement maps cleanly to a few files. Omit when the implementation steps already cover file mapping.]

**Behavior:**
- [Step-by-step behavior description]
- [Include exact values, strings, status codes where known]

**Edge Cases:**
- [Scenario] → [Expected behavior]
- [Scenario] → [Expected behavior]

**Validation Rules:**
- [Rule 1]
- [Rule 2]

**Code Example (optional):** [Include when behavior is hard to express in prose — complex logic, regex patterns, data transformations, or non-obvious API usage.]

**Depends On:** REQ-000 (if applicable)
```

Requirements should be ordered by dependency — an implementing agent should be able to work through them top to bottom.

## Data Model / Schema Changes
- Exact field names, types, defaults, and constraints
- Migration steps if modifying existing data
- Sample data for testing

## API Contracts
- Endpoint, method, request/response shapes
- Status codes and error responses
- Authentication requirements

## UI/UX Specification
- Component hierarchy
- State management approach
- User interaction flow (step by step)
- Loading, empty, and error states
- Accessibility requirements

## Error Handling Strategy
- For each error category: what to show the user, what to log, whether to retry
- Fallback behaviors
- Graceful degradation approach

## Testing Strategy
- What to test (unit, integration, e2e)
- Key test cases mapped to requirements:

| Test Case | Covers | Type |
|-----------|--------|------|
| [Description of test] | REQ-001 | Unit |
| [Description of test] | REQ-001, edge case #1 | Integration |
| [Description of test] | REQ-002 | E2E |

- Test data setup requirements

## Implementation Plan

Break the work into ordered steps that respect context window limits:

```markdown
## Implementation Steps

Each step should be completable in a single focused session.

### Step 1: [Title]
- What to do
- **Files:** path/to/file1.ts, path/to/file2.ts *(include when paths are known)*
- **Done when:** [Machine-verifiable criteria, e.g., "file exists, exports `createUser`, unit tests pass"]

### Step 2: [Title]
- What to do
- Depends on Step 1 being complete
- **Done when:** [e.g., "`POST /api/users` returns 201 with valid payload, 400 on missing fields"]
```

Include file lists per step when working in an existing codebase where paths are known. For greenfield projects or when the structure is still taking shape, omit them — the ordering, scope, and done-when criteria are what matter most.

## Open Questions
- Any remaining uncertainties
- Decisions deferred to implementation time
- Unknowns that could not be resolved during the questioning rounds
