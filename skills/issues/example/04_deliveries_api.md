# 04 — Deliveries API

Depends on: 01

## Context

- When a merchant asks why they never received an event, support has nothing to look at.
- Delivery and attempt rows exist but are reachable only from a console.

## Build

A read endpoint returning a merchant's deliveries, newest first, each carrying its attempts inline.

```
GET /v1/webhook_deliveries
Headers:
  - Authorization: Bearer <merchant api key>   // scopes every result
Query:
  - state: DeliveryState                       // optional
  - event_type: String                         // optional, exact match
  - limit: Integer, 1..100, default 25
Returns:
  - 200: { data: Array<WebhookDeliveryResource> }
  - 401: unauthenticated
  - 422: state or event_type outside the known set
```

```ruby
WebhookDeliveryResource = {
  id:              String,
  event_type:      String,
  endpoint_url:    String,
  state:           DeliveryState,          # pending | delivered | dead
  attempt_count:   Integer,
  next_attempt_at: String,                 # ISO8601, null once delivered or dead
  created_at:      String,
  payload:         WebhookEnvelope,
  attempts:        Array<AttemptResource>  # ascending by number
}

AttemptResource = {
  number:      Integer,
  http_status: Integer,  # null on timeout
  error:       String,   # null on a response
  duration_ms: Integer,
  created_at:  String
}
```

```json
{
  "data": [
    {
      "id": "wd_01H...",
      "event_type": "payment.succeeded",
      "endpoint_url": "https://merchant.example/hooks",
      "state": "dead",
      "attempt_count": 6,
      "next_attempt_at": null,
      "created_at": "2026-09-14T10:02:11Z",
      "payload": { "id": "evt_01H...", "type": "payment.succeeded", "created_at": "...", "data": {} },
      "attempts": [
        { "number": 1, "http_status": 500,  "error": null,               "duration_ms": 812,   "created_at": "2026-09-14T10:02:12Z" },
        { "number": 2, "http_status": null, "error": "execution expired", "duration_ms": 10000, "created_at": "2026-09-14T10:03:14Z" }
      ]
    }
  ]
}
```

## Constraints

- **Attempts come inline, always.** Every consumer needs them, and a second round trip per delivery is what makes a list of fifty unusable.
- **A merchant only ever sees their own deliveries.** Another merchant's delivery is indistinguishable from one that doesn't exist.
- **Ordering is `created_at DESC, id DESC` and is not client-controlled.** The tiebreak matters — a publish writes several rows in one statement with identical timestamps.
- **An unknown `state` or `event_type` is a 422, not an empty list.** An empty list reads as "no deliveries" and hides the typo.
- **Filters combine.** State and event type together narrow to rows matching both.
- **Authentication and scoping follow the existing merchant-facing endpoints,** including how they respond to an unknown id.

## Done when

- A merchant's deliveries come back newest first with attempts inline, under a `data` key.
- Filtering by state returns only that state.
- Filtering by event type returns only that type.
- Both filters together apply together.
- An unknown filter value returns 422.
- `limit` caps the result, defaults to 25 and refuses anything above 100.
- A delivery belonging to another merchant is not readable.
- A delivery with no attempts yet comes back with an empty attempt list, not a missing field.

## Tests

- **Integration** — two deliveries created a minute apart come back newest first.
- **Integration** — two deliveries from one publish, identical timestamps: the order is stable across repeated requests.
- **Integration** — three deliveries, one dead: filtering on dead returns that one.
- **Integration** — state and event type together: only the row matching both.
- **Integration** — `state=exploded` returns 422, not an empty list.
- **Integration** — `limit=200` is refused; `limit` omitted returns at most 25.
- **Integration** — requesting another merchant's delivery id gets the same response this API gives for an unknown id.
- **Integration** — a pending delivery with zero attempts returns `attempts: []`.

## Out of scope

- **Replaying a delivery** — issue 05.
- **Rendering any of this** — issue 06.
- **Cursor pagination** — `limit` is the whole of it here; paging past the cap was left open.
