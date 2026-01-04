# ARCH-3 Crate Map + Dependency Audit

## Purpose
Establish a crate-level decomposition that enables parallel development
by multiple agents, enforces dependency direction, and isolates heavy
adapter/UI dependencies from core logic.

## Target crates (proposed)

| Crate | Scope | Allowed deps |
| --- | --- | --- |
| `rotappo-domain` | Domain models, invariants, enums, snapshot types | std only | 
| `rotappo-ports` | Port traits and domain-facing contracts | domain | 
| `rotappo-application` | Runtime orchestration and state update logic | domain, ports | 
| `rotappo-ui-presentation` | UI/CLI-agnostic formatting + view models | domain | 
| `rotappo-ui-core` | Framework-agnostic UI contracts | domain, ui-presentation | 
| `rotappo-ui-terminal` | Terminal formatting + output modes | domain, ui-presentation | 
| `rotappo-ui-tui` | TUI adapter (ratatui/crossterm) | domain, ports, application, ui-presentation, ui-core | 
| `rotappo-adapter-bootstrappo` | Bootstrappo integration + port impls | domain, ports, bootstrappo | 
| `terminal` (bin) | Terminal composition root | ui-terminal, adapter-bootstrappo, application | 
| `tui` (bin) | TUI composition root | ui-tui, adapter-bootstrappo, application | 

Notes:
- Binaries are the only composition roots that build `Runtime`.
- Adapter crates should not depend on interface crates.

## DAG diagram
- Graphviz source: `docs/architecture/ARCH-3-crate-map.dot`

## Directory layout (grouped)
```
crates/
  core/
    rotappo-domain/
    rotappo-ports/
    rotappo-application/
    rotappo-adapter-bootstrappo/
  ui/
    rotappo-ui-presentation/
    rotappo-ui-core/
    rotappo-ui-terminal/
    rotappo-ui-tui/
```

## Dependency direction rules

Allowed edges:
- `ui-*` -> `ui-presentation` -> `application` -> `domain`
- `ui-tui` -> `domain` (status rendering and snapshot views)
- `ui-tui` -> `ports` (context wiring)
- `application` -> `ports`
- `adapters-*` -> `ports` + `domain`
- `ui-core` must not import `ratatui`, `crossterm`, or terminal helpers
- `terminal` must not import TUI or adapters

Disallowed edges (examples):
- `domain` -> any interface, adapter, or presentation crate
- `ports` -> adapter or UI crates
- `adapters-*` -> `ui-*`

## Module-to-crate mapping (current state)

| Path | Target crate | Notes |
| --- | --- | --- |
| `crates/core/rotappo-domain/src/*` | `rotappo-domain` | Core model types + snapshot state. |
| `crates/core/rotappo-ports/src/*` | `rotappo-ports` | Plan/Health/Log ports. |
| `crates/core/rotappo-application/src/*` | `rotappo-application` | Runtime orchestration. |
| `crates/ui/rotappo-ui-presentation/src/*` | `rotappo-ui-presentation` | Shared formatting + logging config. |
| `crates/ui/rotappo-ui-core/src/*` | `rotappo-ui-core` | Framework-agnostic UI types. |
| `crates/ui/rotappo-ui-terminal/src/*` | `rotappo-ui-terminal` | Terminal formatting + output modes. |
| `crates/ui/rotappo-ui-tui/src/*` | `rotappo-ui-tui` | Ratatui adapter and TUI logic. |
| `crates/core/rotappo-adapter-bootstrappo/src/*` | `rotappo-adapter-bootstrappo` | Port impls + bootstrappo mapping. |
| `src/bin/terminal.rs` | `terminal` | Composition root for terminal. |
| `src/bin/tui.rs` | `tui` | Composition root for TUI. |
| `src/lib.rs` | Workspace root or thin re-export | Prefer thin re-export only. |

## Decoupling candidates (non-tightly bound logic)

1) **BootstrappoBackend wiring**
   - Previous: `BootstrappoBackend` built `Runtime` inside adapters.
   - Now: adapter returns port set only; runtime built in bin crate.

2) **TUI App construction**
   - Previous: `rotappo-ui-tui/app` depended on `BootstrappoBackend`.
   - Now: `App::new` accepts injected `Runtime` + `AppContext`.

3) **Terminal formatting is pure**
   - Confirm `rotappo-ui-terminal` only touches domain + presentation.
   - Keep it adapter-free and suitable for reuse in other CLIs.

4) **UI-core independence**
   - Keep `rotappo-ui-core` free of ratatui/crossterm and terminal deps.
   - Enforced by boundary tests.

## Implementation notes (initial)
- Composition roots now live in `src/bin/terminal.rs` and `src/bin/tui.rs`.
- `BootstrappoBackend` exposes ports only; runtime construction happens in bins.
- TUI `start` accepts injected `Runtime` + `AppContext` instead of a backend.
- Domain types now live in `crates/core/rotappo-domain`.

## Migration checklist (ARCH-3C)
- [x] Add workspace members for target crates.
- [x] Scaffold empty crates under `crates/`.
- [x] Migrate core domain layer into `rotappo-domain`.
- [x] Update imports to use `rotappo_domain` crate paths.
- [x] Migrate ports into `rotappo-ports`.
- [x] Migrate application runtime into `rotappo-application`.
- [x] Migrate presentation helpers into `rotappo-ui-presentation`.
- [x] Migrate UI core contracts into `rotappo-ui-core`.
- [x] Migrate terminal formatting into `rotappo-ui-terminal`.
- [x] Migrate TUI adapter into `rotappo-ui-tui`.
- [x] Migrate bootstrappo adapter into `rotappo-adapter-bootstrappo`.
- [x] Update bins to reference crate-level imports.

## Guardrails and tests

- Add crate boundary checks (deny higher-layer imports in lower crates).
- Keep `tests/interface_boundaries.rs` as a fast guardrail.
- Enforce crate dependency direction in `tests/crate_boundaries.rs`.
- Add CI tasks for `cargo check --no-default-features --features ui-core`.

## Open questions

- Do we want a dedicated `rotappo-composition` crate for shared wiring
  used by both bins (instead of duplicating composition code)?
- Should `rotappo-ui-presentation` be split into `formatting` vs `logging`
  sub-crates, or keep them together for stability?

## Next steps (post-ARCH-3)

1) Decide whether to add a shared composition crate for bin wiring.
2) Consider splitting `rotappo-ui-presentation` into smaller sub-crates.
3) Tighten boundary tests as new adapters/interfaces are added.
