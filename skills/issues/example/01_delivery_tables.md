# 01 — Delivery and attempt tables

## Context

- A webhook that fails delivery is dropped. There is no record the event existed and no way to replay it.
- Retrying needs the event to outlive the request that published it, and support needs to see what was tried.

## Build

Two tables: one row per delivery a merchant is owed, one row per attempt made against it.

```ruby
DeliveryState = "pending" | "delivered" | "dead"
```

```ruby
class CreateWebhookDeliveries < ActiveRecord::Migration[7.1]
  def change
    create_table :webhook_deliveries do |t|
      t.references :merchant, null: false, foreign_key: true
      t.string   :event_type, null: false
      t.string   :endpoint_url, null: false
      t.jsonb    :payload, null: false
      t.string   :state, null: false, default: "pending"  # DeliveryState
      t.integer  :attempt_count, null: false, default: 0
      t.datetime :next_attempt_at, null: false
      t.timestamps
    end
    add_index :webhook_deliveries, [:state, :next_attempt_at]

    create_table :webhook_delivery_attempts do |t|
      t.references :webhook_delivery, null: false, foreign_key: true
      t.integer  :number, null: false
      t.integer  :http_status
      t.string   :error
      t.integer  :duration_ms, null: false
      t.datetime :created_at, null: false
    end
  end
end
```

## Constraints

- **`DeliveryState` is enforced in the model, not the database.** The set changes more often than a migration is worth.
- **The payload column holds the body as it was at publish time.** No foreign keys into it, no recomputing it on read.
- **`[state, next_attempt_at]` is the claim index.** The worker selects on both together; a single-column index doesn't serve it.
- **`http_status` and `error` are both nullable.** A timeout has neither a status nor a response, and it is still an attempt.
- **Attempts are append-only, and `number` is never reused.** A replay continues the numbering rather than restarting it, so the history survives.
- **Migrations are reversible and run in one file,** as every migration in this repo does.

## Done when

- The migration applies clean and rolls back clean.
- A new delivery row defaults to `pending` with `attempt_count` at zero.
- An attempt row cannot exist without its delivery.

## Out of scope

- **Writing delivery rows** — issue 02.
- **Claiming and sending them** — issue 03.
