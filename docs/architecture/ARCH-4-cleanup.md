# ARCH-4 Code Smell + Dead Code Audit

## Purpose
Document the ARCH-4C cleanup pass, list removed dead code, and capture
error-handling and loop-safety conventions for distributed adapters.

## Dead Code Findings + Removal Plan
Removed:
- Unused analytics renderers in `lib/ui/phenome-ui-tui/src/panels/main.rs`.
Adjusted:
- Added retention and scheduler loop entrypoints to match analytics service wiring.

Deferred (documented for follow-up):
- `phenome-adapter-analytics` scheduler execution uses a stub executor.
- `phenome-adapter-notification` InTUI delivery logs to tracing only.
- `phenome-ml` anomaly detection uses Z-score only; Isolation Forest is deferred.

## Error Handling Conventions
- Return `anyhow::Result` at adapter boundaries; add context to IO failures.
- Avoid `expect`/`unwrap` in long-running loops; log and continue or return
  an error instead of panicking.
- Use `tracing::error!` for failed external calls and `tracing::warn!` when
  budgets are exceeded or work is capped.
- Lock poisoning should never panic in adapters; return an error or fall
  back to an empty/default value.

## Backpressure + Loop Safety Checklist
- Explicit tick interval constants for every loop.
- Shutdown signal or exit condition (shutdown channel, closed sender).
- Per-tick work caps (`MAX_*`) and drop/deferral logging.
- Timeout budgets for long-running external calls.
- Bounded queues for UI/adapter update channels.
- Avoid unbounded drain loops; cap and log when capped.

## Testing Checklist Updates
See `docs/book/testing.md` for the updated cleanup verification list.
