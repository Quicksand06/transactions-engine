# Transaction Engine

A small streaming-friendly payments engine that processes CSV transactions
and outputs final account balances.

## Build
cargo build

## Run
cargo run -- input/transactions.csv > accounts.csv

## Notes
An Event Sourcing pattern (or a lightweight custom variant of it) is used for this task.

### Architecture Idea
- Each input line is treated as a **command**
- Valid commands produce **events**
- Events are applied to a `ClientAccount` aggregate
- A projection derives the final account balances

### Why Event Sourcing
**Pros**
- Full audit trail (why is the account in this state?)
- Very testable (given a sequence of events → expect a final state)
- Natural fit for dispute / resolve / chargeback workflows
- Clear separation between validation and state mutation

**Cons**
- More moving parts (commands, events, projections)
- Slightly more boilerplate for a small CLI tool

### Correctness
The event-based design makes correctness easy to validate by replaying events
and asserting the resulting account state. State transitions are explicit and
deterministic.

### Efficiency
Transactions are processed line-by-line without loading the full CSV into memory,
making the engine suitable for large input files or streaming sources.

### Assumptions
- Disputes, resolves, and chargebacks apply only to existing deposit transactions
- Invalid or unknown transaction references are ignored as defined by the specification
- Duplicate dispute / resolve commands have no effect
- Clients are created on the fly on first occurrence

### What I Haven’t Done
- Events are currently stored in memory and not persisted, as the CLI is executed per request.
  They nevertheless act as a single source of truth from which projections can be derived.
  In a real-world scenario, events would be persisted or published to a message broker and
  processed asynchronously by projection handlers, providing eventual consistency.
- Error handling is intentionally minimal. Invalid lines are ignored according to the
  specification, which is enough for the scope of this task.

### Concurrency note: 
The CLI processes a single input stream sequentially. If exposed as a web service, per-request processing remains trivially safe. 
For a shared global ledger, commands would need to be funneled through a single-writer/actor (or protected with locks) and enforce idempotency and atomic state transitions to avoid races across concurrent streams.