# Example Spec Output

This is a condensed example showing the expected format and tone. A real spec would be more detailed.

---

> **TL;DR:** Adds a password reset flow via email with token-based verification. 5 requirements, touches 8 files (3 new, 5 modified). Constrained to existing Express + Postgres stack with Resend for email.
>
> **Done when:** E2E test completes full reset flow (request → email → new password → login), all endpoints return expected status codes, migration is reversible, no existing tests broken.

## 1. Overview

Users currently have no way to recover access if they forget their password. This spec adds a password reset flow: user requests a reset link via email, receives a tokenized link, and sets a new password through a dedicated form.

**Related documents:**
- Prior art: `src/routes/auth.ts` — existing login/register endpoints (follow the same route pattern)
- Prior art: `src/emails/welcome.tsx` — existing email template (follow the same component structure)

## 2. Goals & Success Criteria

- Users can regain account access without admin intervention
- Reset tokens expire after 1 hour and are single-use
- Password reset emails are delivered within 30 seconds of request
- Success verified by: e2e test completing the full reset flow

## 3. Non-Goals & Out of Scope

- Account lockout after failed attempts (separate spec)
- Password strength meter UI (nice-to-have, not in this iteration)
- SMS-based reset

## 4. Constraints & Assumptions

- Uses existing Express 4.x backend and Postgres 15 database
- Email delivery via Resend (already configured for transactional email)
- Tokens are SHA-256 hashed before storage (plaintext never persisted)
- Frontend uses React 18 with React Router 6 — existing pattern

## 5. Agent Boundaries

- **Always:** Run existing test suite after each implementation step. Use existing `isValidEmail` util for email validation. Follow existing Express route patterns in `src/routes/`.
- **Ask first:** Adding new npm dependencies. Modifying the `users` table schema. Changing existing auth middleware.
- **Never:** Store plaintext tokens in the database. Log email addresses at debug level. Modify existing password hashing logic in other files. Delete or alter the `users` table.

## 6. Architecture & Approach

Token-based reset via email. The reset token is generated server-side, hashed, and stored in a new `password_reset_tokens` table. The plaintext token is sent via email as a URL parameter. On submission, the server verifies the hash, checks expiry, and updates the password.

**File manifest:**

| File | Action | Purpose |
|------|--------|---------|
| `src/db/migrations/004_password_reset_tokens.sql` | Create | New table for reset tokens |
| `src/routes/auth.ts` | Modify | Add POST `/forgot-password` and POST `/reset-password` |
| `src/services/password-reset.ts` | Create | Token generation, hashing, verification logic |
| `src/emails/password-reset.tsx` | Create | Email template for reset link |
| `src/pages/ForgotPassword.tsx` | Create | Request reset form |
| `src/pages/ResetPassword.tsx` | Create | New password form |
| `src/routes/index.ts` | Modify | Register new routes |
| `tests/password-reset.test.ts` | Create | Unit + integration tests |

## 7. Detailed Requirements

### REQ-001: Password reset request endpoint

**What:** POST `/api/forgot-password` accepts an email, generates a reset token, and sends an email.

**Where:** `src/routes/auth.ts`, `src/services/password-reset.ts`

**Behavior:**
- Accept `{ email: string }` in request body
- If email exists in `users` table: generate 32-byte random token, SHA-256 hash it, store hash + user_id + expires_at (now + 1 hour) in `password_reset_tokens`, send email with plaintext token in URL
- If email does not exist: return 200 with same response (prevent email enumeration)
- Response: `{ message: "If an account exists with that email, a reset link has been sent." }`
- Invalidate any existing unexpired tokens for that user before creating a new one

**Edge Cases:**
- User requests reset twice in 30 seconds → second request replaces the first token; first link becomes invalid
- Email delivery fails → log error, return 200 anyway (do not leak failure to client)
- Invalid email format → return 400 with validation error

**Validation Rules:**
- Email must be valid format (use existing `isValidEmail` util)
- Request body must contain `email` field

**Code Example:**
```typescript
import { randomBytes, createHash } from 'crypto';

const token = randomBytes(32).toString('hex');           // plaintext — sent in email
const tokenHash = createHash('sha256').update(token).digest('hex'); // stored in DB
```

### REQ-002: Password reset submission endpoint

**What:** POST `/api/reset-password` accepts a token and new password, verifies the token, and updates the password.

**Where:** `src/routes/auth.ts`, `src/services/password-reset.ts`

**Behavior:**
- Accept `{ token: string, password: string }` in request body
- SHA-256 hash the provided token, look up matching row in `password_reset_tokens`
- If found and not expired: update user's password (bcrypt hash, 12 rounds), delete the token row, return 200
- If not found or expired: return 400 `{ error: "Invalid or expired reset link." }`

**Edge Cases:**
- Token used twice → second attempt fails (token deleted after first use)
- Token is valid but user account was deleted between request and submission → return 400 with same generic error
- Password identical to current password → allow it (no restriction)

**Validation Rules:**
- Password minimum 8 characters
- Token must be 64-character hex string (32 bytes hex-encoded)

**Depends On:** REQ-001

## 8. Data Model / Schema Changes

```sql
CREATE TABLE password_reset_tokens (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token_hash VARCHAR(64) NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_reset_tokens_hash ON password_reset_tokens(token_hash);
CREATE INDEX idx_reset_tokens_user ON password_reset_tokens(user_id);
```

## 9. Error Handling Strategy

| Error | User sees | Logged | Retry |
|-------|-----------|--------|-------|
| Email delivery failure | Generic success message | Yes (error level) | No |
| Database error on token creation | 500 "Something went wrong" | Yes (error level) | No |
| Invalid/expired token | 400 "Invalid or expired reset link" | Yes (warn level) | User can request new link |
| Password hash failure | 500 "Something went wrong" | Yes (error level) | No |

## 10. Testing Strategy

| Test Case | Covers | Type |
|-----------|--------|------|
| Valid email triggers token creation and email send | REQ-001 | Integration |
| Unknown email returns 200 (no enumeration) | REQ-001, edge case | Unit |
| Valid token resets password | REQ-002 | Integration |
| Expired token returns 400 | REQ-002, edge case | Unit |
| Used token cannot be reused | REQ-002, edge case | Integration |
| Full flow: request → email → reset → login with new password | REQ-001, REQ-002 | E2E |

## 11. Implementation Plan

### Step 1: Database migration
- Create `password_reset_tokens` table
- **Files:** `src/db/migrations/004_password_reset_tokens.sql`
- **Done when:** Migration runs successfully, table exists with correct schema, indexes present

### Step 2: Token service
- Implement token generation, hashing, storage, and verification
- **Files:** `src/services/password-reset.ts`
- **Done when:** Unit tests pass for: generate token, hash token, store token, verify valid token, reject expired token, reject missing token

### Step 3: API endpoints
- Add forgot-password and reset-password routes
- **Files:** `src/routes/auth.ts`, `src/routes/index.ts`
- **Done when:** `POST /api/forgot-password` returns 200 for known and unknown emails; `POST /api/reset-password` returns 200 for valid token, 400 for invalid
- Depends on Step 2

### Step 4: Email template
- Create password reset email with tokenized link
- **Files:** `src/emails/password-reset.tsx`
- **Done when:** Email renders correctly with reset URL containing token parameter
- Depends on Step 2

### Step 5: Frontend pages
- Create ForgotPassword and ResetPassword pages with forms
- **Files:** `src/pages/ForgotPassword.tsx`, `src/pages/ResetPassword.tsx`
- **Done when:** Forms submit to correct endpoints, display success/error states, validate input client-side
- Depends on Step 3

### Step 6: E2E test
- Full flow from request to login with new password
- **Files:** `tests/password-reset.test.ts`
- **Done when:** E2E test passes: request reset → verify email sent → submit new password → login succeeds with new password

## 12. Open Questions

- None — all questions resolved during spec creation.
