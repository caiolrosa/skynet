# 05 — Replay endpoint

Depends on: 03, 04

## Context

- A delivery that exhausted its retries is dead forever. The merchant fixes their endpoint and the event is still gone.
- Recovering one today means editing a row by hand in a console.

## Build

An endpoint that returns a dead delivery to pending so the worker picks it up on its next pass.

```
POST /v1/webhook_deliveries/:id/replay
Headers:
  - Authorization: Bearer <merchant api key>
Body: none
Returns:
  - 200: WebhookDeliveryResource        // state is "pending" again
  - 401: unauthenticated
  - 404: no such delivery for this merchant
  - 409: state is not "dead"
```

```ruby
Webhooks::Replay.call(delivery)  # => WebhookDelivery
```

```
-> POST /v1/webhook_deliveries/:id/replay
  -> Api::V1::WebhookDeliveryReplaysController#create
    -> authenticate_merchant!
    -> Webhooks::Replay.call
      -> delivery.update!(state:, attempt_count:, next_attempt_at:)
    -> WebhookDeliverySerializer.render
```

## Constraints

- **Only a dead delivery is replayable, and anything else is a 409.** Replaying a pending one double-sends; replaying a delivered one resends what the merchant already has.
- **Replay resets `attempt_count` to zero and makes the delivery due now.** It gets a full six attempts again.
- **The old attempts are not deleted, and `number` continues from the highest recorded.** After a replay `attempt_count` is lower than the attempt count — the first is the backoff position, the second is the history.
- **The payload is untouched.** The merchant receives what the event said when it happened, not what the record says today.
- **The response returns before anything is sent.** The worker delivers on its next pass, and the replayed send is recorded as an ordinary attempt.
- **Another merchant's delivery is a 404, never a 403.** A 403 confirms the id exists.

## Done when

- Replaying a dead delivery makes it pending, with attempt count zero and next attempt due now.
- The worker claims a replayed delivery on its next pass.
- Replaying a pending delivery returns 409 and changes nothing.
- Replaying a delivered delivery returns 409 and changes nothing.
- The send that follows a replay is recorded as an attempt, numbered after the ones already there.
- Another merchant's delivery returns 404.

## Tests

- **Integration** — a dead delivery with `attempt_count` 6 replays to pending, count 0, due now.
- **Integration** — after replay, the worker's next pass claims it and POSTs the original stored payload.
- **Integration** — the attempt recorded after a replay is numbered 7, and attempts 1 through 6 are still present.
- **Integration** — replaying a pending delivery returns 409; state and attempt count are unchanged.
- **Integration** — replaying a delivered delivery returns 409; state and attempt count are unchanged.
- **Integration** — replaying another merchant's delivery returns 404.

## Out of scope

- **The replay button** — issue 06.
- **Bulk replay of every dead delivery for an endpoint** — not asked for, and one merchant's outage would make it a thousand sends in a second.
