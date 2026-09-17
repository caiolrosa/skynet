# 04 — Deliveries API

Depends on: 01

## Context

- When a merchant asks why they never received an event, support has nothing to look at.
- Delivery and attempt rows exist but are reachable only from a console.

## Build

A read endpoint returning a merchant's deliveries, newest first, each carrying its attempts inline.

```json
{
  "deliveries": [
    {
      "id": "wd_01H...",
      "event_type": "payment.succeeded",
      "endpoint_url": "https://merchant.example/hooks",
      "state": "dead",
      "attempt_count": 6,
      "next_attempt_at": null,
      "created_at": "2026-09-14T10:02:11Z",
      "attempts": [
        {
          "number": 1,
          "http_status": 500,
          "error": null,
          "duration_ms": 812,
          "created_at": "2026-09-14T10:02:12Z"
        },
        {
          "number": 2,
          "http_status": null,
          "error": "execution expired",
          "duration_ms": 10000,
          "created_at": "2026-09-14T10:03:14Z"
        }
      ]
    }
  ]
}
```

## Constraints

- **Attempts come inline, always.** The dashboard shows them expanded by default, and a second call is a call that gets skipped.
- **A merchant only ever sees their own deliveries.** Another merchant's delivery is indistinguishable from one that doesn't exist.
- **Filters are state and event type,** and they combine.
- **Ordering is newest first** and is not client-controlled.
- **Authentication and scoping follow the existing merchant-facing endpoints,** including how they respond to an unknown id.

## Done when

- A merchant's deliveries come back newest first with attempts inline.
- Filtering by state returns only that state.
- Filtering by event type returns only that type.
- Both filters together apply together.
- A delivery belonging to another merchant is not readable.
- A delivery with no attempts yet comes back with an empty attempt list, not a missing field.

## Tests

- **Integration** — two deliveries created a minute apart come back newest first.
- **Integration** — three deliveries, one dead: filtering on dead returns that one.
- **Integration** — state and event type together: only the row matching both.
- **Integration** — requesting another merchant's delivery id gets the same response this API gives for an unknown id.
- **Integration** — a pending delivery with zero attempts returns `attempts: []`.

## Out of scope

- **Replaying a delivery** — issue 05.
- **Rendering any of this** — issue 06.
