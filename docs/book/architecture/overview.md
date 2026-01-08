# Architecture Overview

Rotappo follows a layered design:

1) domain: pure models and invariants
2) application: orchestration and runtime flows
3) ports: interfaces defined in domain terms
4) adapters: external systems implementing ports
5) presentation: UI/CLI-agnostic formatting and view helpers
6) interfaces: CLI/TUI rendering and I/O

Dependencies flow inward only:

ui-* -> ui-presentation -> application -> domain
application -> ports
adapters -> ports + domain
ui-terminal -> adapter (bootstrappo CLI handlers)

Crate layout (in progress):
- `crates/core/rotappo-domain` (domain types)
- `crates/core/rotappo-ports` (ports)
- `crates/core/rotappo-application` (runtime orchestration)
- `crates/core/rotappo-adapter-bootstrappo` (bootstrappo adapter)
- `crates/ui/rotappo-ui-presentation` (formatting/logging helpers)
- `crates/ui/rotappo-ui-core` (framework-agnostic UI contracts)
- `crates/ui/rotappo-ui-terminal` (CLI formatting + dispatch)
- `crates/ui/rotappo-ui-tui` (ratatui adapter)

Directional rules are enforced by `tests/crate_boundaries.rs` and
interface checks in `tests/interface_boundaries.rs`.

Composition roots:
- `src/bin/cli.rs` (bootstrappo CLI entrypoint)
- `src/bin/tui.rs` (TUI)
