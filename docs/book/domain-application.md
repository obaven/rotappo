# Domain and Application

The domain layer (`crates/core/rotappo-domain/`) contains the core models and their
invariants. These types must stay free of UI, CLI, and adapter details.

The application layer (`crates/core/rotappo-application/`) coordinates ports, adapters,
and domain state. It is the runtime orchestration boundary.

Typical flow:
- adapters provide data via ports
- application builds domain snapshots
- presentation formats snapshots for interfaces
