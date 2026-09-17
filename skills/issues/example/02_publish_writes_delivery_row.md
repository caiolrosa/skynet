# 02 — Publishing writes a delivery row

Depends on: 01

## Context

- Publishing an event POSTs to the merchant inline, inside the request that caused it. A failure is lost the moment the request ends.
- The request also pays the merchant's latency, so a slow endpoint slows down our own API.

## Build

Publishing an event writes one pending delivery row per configured endpoint and returns. Nothing leaves the process on the request path.

```ruby
# one row per configured endpoint, ready to be claimed immediately
WebhookDelivery.create!(
  merchant:        merchant,
  event_type:      "payment.succeeded",
  endpoint_url:    endpoint.url,
  payload:         serialized_event,   # frozen here, never recomputed
  next_attempt_at: Time.current
)
```

## Constraints

- **The payload is frozen at insert.** A replay six hours later sends what the event said then, not what the record says now.
- **The body is byte-for-byte what the old inline path sent.** Merchants have parsers built against it, and this change is invisible to them.
- **No outbound HTTP on the request path.** Not a fast one, not a fire-and-forget one.
- **A merchant with no configured endpoint produces no rows,** and publishing still succeeds.
- **`next_attempt_at` is set to now,** so the first attempt happens on the worker's next pass rather than after a delay.

## Done when

- Publishing an event inserts one pending row per configured endpoint.
- The stored payload matches what the old inline path sent, byte for byte.
- Publishing makes no outbound HTTP call.
- Publishing for a merchant with no endpoints inserts nothing and raises nothing.

## Tests

- **Integration** — publishing for a merchant with two endpoints inserts two pending rows, one per URL.
- **Integration** — publishing makes no outbound call: any HTTP from the request path fails the test.
- **Integration** — the stored payload for `payment.succeeded` equals the body the inline path sent, compared against a known-good literal.
- **Integration** — publishing for a merchant with no endpoints inserts nothing and returns normally.

## Out of scope

- **Sending the rows** — issue 03.
- **Removing the old inline path's dead code** — it stops being reached here; deleting it is not part of this work.
