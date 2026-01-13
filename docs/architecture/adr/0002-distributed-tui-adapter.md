# ADR-0002: Distributed TUI Adapter Design + Loop Safety

## Status
Accepted (2026-01-06)

## Context
The TUI adapter orchestrates multiple tool adapters (bootstrappo today) and
relies on long-running loops for polling, notifications, and scheduling.
Without clear backpressure and exit rules, these loops risk runaway work,
missed frames, and ambiguous ownership.

## Decision
Adopt the distributed TUI design principles and loop safety rules:
- `docs/architecture/ARCH-4-distributed-tui-design.md` is the source of
  adapter design principles.
- Long-running loops must have explicit tick intervals, budgets, and exit
  conditions (shutdown channel or closed sender).
- Per-tick caps are required for heavy data flows to protect frame time.

## Consequences
- Adapter loops must log when budgets are exceeded and defer work.
- UI update drains are capped to avoid starvation.
- Runbooks and tests must reference the loop safety checklist.

## Alternatives considered
- Allow unbounded loops and rely on external throttling (rejected).
- Centralize all loops in a single runtime service (deferred; revisit later).

## Links
- `docs/architecture/ARCH-4-distributed-tui-design.md`
- `docs/architecture/ARCH-4-cleanup.md`
- `docs/book/testing.md`
