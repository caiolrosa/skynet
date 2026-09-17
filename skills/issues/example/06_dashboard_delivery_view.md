# 06 — Dashboard delivery view

Depends on: 04, 05

## Context

- Merchants find out about a failed webhook by reconciling payouts against their own ledger, days later.
- The delivery history exists over the API and nothing in the dashboard shows it.

## Build

A deliveries list in the merchant dashboard: what was sent, what happened, and a way to replay what died.

The endpoints this consumes, in full — nothing here calls anything else:

```json
// GET /v1/webhook_deliveries?state=dead&event_type=payment.succeeded
{
  "deliveries": [
    {
      "id": "wd_01H...",
      "event_type": "payment.succeeded",
      "endpoint_url": "https://merchant.example/hooks",
      "state": "pending | delivered | dead",
      "attempt_count": 6,
      "next_attempt_at": "2026-09-17T14:22:03Z",  // null once dead or delivered
      "created_at": "2026-09-14T10:02:11Z",
      "attempts": [
        { "number": 1, "http_status": 500, "error": null, "duration_ms": 812, "created_at": "..." },
        { "number": 2, "http_status": null, "error": "execution expired", "duration_ms": 10000, "created_at": "..." }
      ]
    }
  ]
}

// POST /v1/webhook_deliveries/:id/replay  — dead only
// 200 → the delivery in the shape above, now pending with attempt_count 0
```

## Constraints

- **The list shows state, event type, endpoint and last attempt result.** The last attempt is the newest entry in `attempts`, and a delivery with none shows no result rather than a blank cell.
- **An attempt with a null status is a timeout,** and shows its error text rather than an empty status column.
- **Replay appears only on dead deliveries.** Every other state has no button, not a disabled one.
- **A replayed delivery shows its new state without a reload.**
- **Attempts render expanded.** The data is already in the response, and a click to reveal it is a click most people don't make.
- **The state filter drives the request,** rather than filtering rows already fetched.

## Done when

- The list renders state, event type, endpoint and last attempt result for a merchant's deliveries.
- Attempts show expanded, with status, error and duration.
- A timeout attempt shows its error rather than an empty status.
- The state filter refetches and shows only that state.
- Replay shows only on dead rows.
- Replaying updates the row to pending without a reload.

## Tests

- **Integration** — a delivery with two attempts renders both, expanded, with duration.
- **Integration** — an attempt with a null status renders its error text.
- **Integration** — selecting "dead" issues a request filtered to dead and renders what comes back.
- **Integration** — a dead row shows replay; pending and delivered rows have no button at all.
- **Integration** — clicking replay on a dead row leaves the row showing pending, with no reload.

## Out of scope

- **Changing what either endpoint returns** — issues 04 and 05 own their shapes.
- **Filtering by date range** — nobody asked for it.
