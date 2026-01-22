# ARCH-4 Distributed TUI System Design Principles

## Purpose
Define world-class system design principles for the distributed TUI adapter
that speaks to multi-tool hex apps (bootstrappo today). This document focuses
on latency, resilience, state synchronization, and extensibility so the TUI
remains responsive as more tools and adapters come online.

## Scope
- Applies to the TUI adapter in `lib/ui/phenome-ui-tui`.
- Assumes the hex boundary pattern: interface adapters talk to tool ports.
- Bootstrappo is the current tool adapter; additional tools should follow the
  same contract.

## Non-goals
- Replacing the render stack or terminal backend.
- Redesigning the application layer or port shapes.
- Implementing a full UI-core extraction (tracked elsewhere).

## System Model
### Components
- TUI adapter: event loop, input handling, render pipeline, view state.
- Tool ports: typed access to snapshot, logs, actions, and metadata.
- Runtime: orchestrates ports and provides a stable snapshot surface.
- Tool adapters: bootstrappo today, future tools later.

### Component diagram (ASCII)
```
[User Input] -> [TUI Adapter] -> [Ports] -> [Tool Adapter]
                         ^          |
                         |          v
                   [Runtime Snapshot + Events]
```

### High-level flow
1) Input -> TUI adapter -> action requests via ports.
2) Tool adapters update runtime state -> cached in TUI state.
3) Render loop consumes cached state -> draws frame.

## Design Principles
### 1) Deterministic render loop
- Render loop should never block on network or long-running IO.
- All port access in render should be read-only and fast (cached or in-memory).
- Heavy work (graph render, analytics fetch) happens out-of-band and feeds
  caches with timestamps.

### 2) Single source of truth for UI state
- UI state is owned by the adapter (`UiState`), not tool adapters.
- Tool state is pulled or streamed into caches with explicit freshness rules.

### 3) Backpressure by design
- All streams must have caps and drop policies.
- Log/event buffers must be bounded with a deterministic eviction rule.
- Update loops must coalesce rapid updates into a single frame.

### 4) Graceful degradation
- If a tool port is slow or unavailable, show stale data with a clear banner.
- If graph rendering fails, fall back to ASCII views without blocking input.
- Input handling always remains responsive even during heavy redraws.

### 5) Hex boundary enforcement
- TUI adapter speaks only to ports, not tool adapter internals.
- No cross-layer imports from adapters into UI modules.
- Tool-specific affordances live behind port abstractions.

### 6) Extensibility first
- Adding a new tool should require only a new port implementation and view
  wiring, not a new adapter architecture.
- Shared UI patterns are exposed via documented helpers/macros.

## Adapter Contract (Bootstrappo today)
Minimum expectations for each tool port:
- Snapshot access: cheap, fast, and consistent within a frame.
- Event stream: bounded drain, monotonic timestamps, and clear severity.
- Action dispatch: idempotent where possible, with explicit confirmations.
- Metadata: tool and module specs available for details panels.
- Failure surface: structured errors for unavailable or stale data.

## State, Caching, and Sync
- Cache snapshots with a timestamp and a refresh interval budget.
- Cache logs/events with a fixed maximum and insert-order eviction.
- Do not mutate tool state in render paths.
- All "live" data flows through ports; UI state stores only view concerns.

## Performance Budgets
- Target frame time: <= 50 ms average, <= 100 ms worst case.
- Input handling: <= 16 ms to update view state.
- Tick rate: 200 ms default, adjustable per view.
- Event throughput: bounded by configured log buffer size.

## Failure Modes and Recovery
- Port unavailable: show stale state and a warning banner; retry on tick.
- Graph render failure: keep last good image or fall back to text.
- Event overflow: drop oldest entries first and surface a warning.
- Action failures: log event with severity and surface a non-blocking alert.

## Documentation and Ownership
- This document is the canonical reference for distributed TUI design.
- ADRs must reference these principles when changing adapter boundaries.
- New tools must add a short "adapter contract" note in interface docs.
- ADR decision record: `docs/architecture/adr/0002-distributed-tui-adapter.md`.
