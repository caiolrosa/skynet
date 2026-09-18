# 06 — Dashboard delivery view

Depends on: 04, 05

## Context

- Merchants find out about a failed webhook by reconciling payouts against their own ledger, days later.
- The delivery history exists over the API and nothing in the dashboard shows it.

## Build

A deliveries list in the merchant dashboard: what was sent, what happened, and a way to replay what died.

```ts
type DeliveryRow = {
  id: string
  eventType: string
  endpointUrl: string
  state: "pending" | "delivered" | "dead"
  lastAttempt: Attempt | null   // null before the first attempt
}

type Attempt = {
  number: number
  httpStatus: number | null     // null on timeout
  error: string | null          // null on a response
  durationMs: number
  createdAt: string
}

type DeliveryFilter = {
  state?: "pending" | "delivered" | "dead"
  eventType?: string
}
```

```ts
fetchDeliveries(filter: DeliveryFilter): Promise<DeliveryRow[]>
replayDelivery(id: string): Promise<DeliveryRow>

<DeliveryList />
<DeliveryListRow delivery={DeliveryRow} />
<ReplayButton deliveryId={string} />
```

```
-> <DeliveryList />
  -> fetchDeliveries
    -> GET /v1/webhook_deliveries
  -> <DeliveryListRow /> per delivery
    -> <ReplayButton />            // rendered only when state is "dead"
      -> replayDelivery
        -> POST /v1/webhook_deliveries/:id/replay
```

The endpoints this consumes, in full — nothing here calls anything else:

```
GET /v1/webhook_deliveries?state=dead&event_type=payment.succeeded&limit=25
  200 -> { "data": [ { id, event_type, endpoint_url, state, attempt_count,
                       next_attempt_at, created_at, payload,
                       attempts: [ { number, http_status, error, duration_ms, created_at } ] } ] }
  422 -> unknown state or event_type

POST /v1/webhook_deliveries/:id/replay
  200 -> one delivery in the shape above, now pending with attempt_count 0
  409 -> the delivery is not dead
  404 -> not this merchant's delivery
```

## Constraints

- **The row shows state, event type, endpoint and the last attempt's result.** The last attempt is the highest-numbered entry in `attempts`, and a delivery with none shows no result rather than a blank cell.
- **An attempt with a null status is a timeout,** and shows its error text rather than an empty status column.
- **Replay appears only on dead deliveries.** Every other state has no button, not a disabled one — the other states are 409s, and a button that always errors is worse than no button.
- **A successful replay swaps the row from the 200 body.** No refetch, no reload.
- **A 409 on replay means another tab already replayed it.** The row refetches instead of showing an error.
- **The state filter drives the request,** rather than filtering rows already fetched. The list is capped at `limit`, so filtering what arrived would hide matching rows.

## Done when

- The list renders state, event type, endpoint and last attempt result for a merchant's deliveries.
- A timeout attempt shows its error rather than an empty status.
- The state filter refetches and shows only that state.
- Replay shows only on dead rows.
- Replaying updates the row to pending without a reload.
- A 409 from replay refetches the row rather than surfacing an error.

## Tests

- **Integration** — a delivery with two attempts renders the second one's result.
- **Integration** — an attempt with a null status renders its error text.
- **Integration** — a delivery with no attempts renders no result, not an empty cell.
- **Integration** — selecting "dead" issues a request filtered to dead and renders what comes back.
- **Integration** — a dead row shows replay; pending and delivered rows have no button at all.
- **Integration** — clicking replay on a dead row leaves the row showing pending, with no reload.
- **Integration** — replay returning 409 refetches that row and shows its current state.

## Out of scope

- **Changing what either endpoint returns** — issues 04 and 05 own their shapes.
- **Showing the stored payload** — the API returns it; rendering it is not asked for here.
- **Filtering by date range** — nobody asked for it.
