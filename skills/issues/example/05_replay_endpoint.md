# 05 — Replay endpoint

Depends on: 03, 04

## Context

- A delivery that exhausted its retries is dead forever. The merchant fixes their endpoint and the event is still gone.
- Recovering one today means editing a row by hand in a console.

## Build

An endpoint that returns a dead delivery to pending so the worker picks it up on its next pass.

```json
POST /v1/webhook_deliveries/:id/replay

// 200 — the delivery, in the same shape the list endpoint returns
{ "id": "wd_01H...", "state": "pending", "attempt_count": 0, "next_attempt_at": "2026-09-17T14:22:03Z" }
```

## Constraints

- **Only a dead delivery is replayable.** Replaying a pending one double-sends; replaying a delivered one resends what the merchant already has.
- **Replay resets the attempt count and makes the delivery due now.** It gets a full six attempts again.
- **The payload is untouched.** The merchant receives what the event said when it happened, not what the record says today.
- **The replayed send is recorded as an attempt like any other,** so the history shows every send the merchant received.
- **Scoping matches the read endpoint.** A merchant cannot replay another merchant's delivery, and the refusal looks like an unknown id.
- **Rejection uses this repo's existing error response shape** for an operation that isn't valid in the current state.

## Done when

- Replaying a dead delivery makes it pending, with attempt count zero and next attempt due now.
- The worker claims a replayed delivery on its next pass.
- Replaying a pending delivery is rejected and changes nothing.
- Replaying a delivered delivery is rejected and changes nothing.
- The send that follows a replay is recorded as an attempt.
- Another merchant's delivery cannot be replayed.

## Tests

- **Integration** — a dead delivery with `attempt_count` 6 replays to pending, count 0, due now.
- **Integration** — after replay, the worker's next pass claims it and POSTs the original stored payload.
- **Integration** — replaying a pending delivery is rejected; state and attempt count are unchanged.
- **Integration** — replaying a delivered delivery is rejected; state and attempt count are unchanged.
- **Integration** — replaying another merchant's delivery gets the unknown-id response.

## Out of scope

- **The replay button** — issue 06.
- **Bulk replay of every dead delivery for an endpoint** — not asked for, and one merchant's outage would make it a thousand sends in a second.
