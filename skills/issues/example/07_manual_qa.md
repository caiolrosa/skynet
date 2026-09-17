# 07 — Manual QA

Depends on: 01, 02, 03, 04, 05, 06

## Context

- Every other issue proves its own piece. Nothing so far proves an event published by the API reaches a merchant, survives their outage, and can be recovered from the dashboard.
- This is the last check before the work ships.

## Build

Run these by hand against a deployed environment, in order. Each one says what to do and what should be true afterwards.

## Constraints

- **Run against a merchant account with two endpoints configured,** one healthy and one pointed at a server you control.
- **These checks are the natural seed for an end-to-end suite.** Whoever automates them later starts here.

## Done when

- Publishing an event to an endpoint that returns 200 shows as delivered with one attempt.
- Pointing an endpoint at a server returning 500 shows attempts accumulating on the 1m, 5m, 25m, 2h backoff schedule.
- A delivery reaching its sixth failure shows as dead, and one alert fires naming the merchant and the endpoint.
- Taking a second endpoint for the same merchant to dead within the hour fires no second alert.
- Replaying the dead delivery against a now-healthy endpoint delivers it, and the replayed send appears in the attempt history.
- A delivered delivery has no replay button.
- Logging in as a different merchant shows none of the above deliveries.
- Publishing an event for a merchant with no configured endpoint succeeds, with no delivery row and no error.

## Out of scope

- **Load testing the worker** — queue depth at current volume is an assumption the team accepted, not something this check settles.
