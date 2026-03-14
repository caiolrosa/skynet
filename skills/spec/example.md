# Example Spec Output

This is a condensed example showing the expected format and tone. A real spec would be more detailed.

---

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

The reset token is generated server-side, hashed, and stored in a new `password_reset_tokens` table. The plaintext token is sent via email as a URL parameter. On submission, the server verifies the hash, checks expiry, and updates the password.

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

## 8. Open Questions

- None — all questions resolved during spec creation.
