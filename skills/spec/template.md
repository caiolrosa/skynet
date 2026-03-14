# Spec Template

Use this template when generating the spec in Phase 6. Only include sections that are relevant to the feature — omit any that don't apply. Number sections sequentially in the generated spec based on which sections are actually included — don't leave numbering gaps from omitted sections.

---

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
- **Never:** [Hard stops — e.g., "Delete existing data or tables not mentioned in this spec", "Modify files outside the specified requirements without explicit approval", "Skip error handling for any endpoint"]

## Architecture & Approach
- Key architectural decisions and WHY they were made

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

## Open Questions
- Any remaining uncertainties
- Decisions deferred to implementation time
- Unknowns that could not be resolved during the questioning rounds
