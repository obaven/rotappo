# ARCH-3 Crate Map + Dependency Audit

## Purpose
Establish a crate-level decomposition that enables parallel development
by multiple agents, enforces dependency direction, and isolates heavy
adapter/UI dependencies from core logic.

## Target crates (proposed)

| Crate | Scope | Allowed deps |
| --- | --- | --- |
| `phenome-domain` | Domain models, invariants, enums, snapshot types | std only | 
| `phenome-ports` | Port traits and domain-facing contracts | domain | 
| `phenome-application` | Runtime orchestration and state update logic | domain, ports | 
| `phenome-ml` | ML model implementations | domain | 
| `phenome-ui-presentation` | UI/CLI-agnostic formatting + view models | domain | 
| `phenome-ui-core` | Framework-agnostic UI contracts | domain, ui-presentation | 
| `phenome-ui-terminal` | CLI formatting + command dispatch | domain, ui-presentation, adapter-bootstrappo | 
| `phenome-ui-tui` | TUI adapter (ratatui/crossterm) | domain, ports, application, ui-presentation, ui-core | 
| `phenome-adapter-primer` | Bootstrappo integration + port impls | domain, ports, bootstrappo | 
| `phenome-adapter-analytics` | Analytics service adapter | domain, ports | 
| `phenome-adapter-ml` | ML service adapter | domain, ports, phenome-ml | 
| `bootstrappo` (bin) | Bootstrappo CLI entrypoint | ui-terminal | 
| `tui` (bin) | TUI composition root | ui-tui, adapter-bootstrappo, application | 
| `analytics-service` (bin) | Analytics service binary | adapter-analytics | 
| `ml-service` (bin) | ML service binary | adapter-ml, phenome-ml | 

Notes:
- Binaries are the only composition roots that build `Runtime`.
- Adapter crates should not depend on interface crates.

## DAG diagram
- Graphviz source: `docs/architecture/ARCH-3-crate-map.dot`

## Directory layout (grouped)
```
lib/
  core/
    phenome-domain/
    phenome-ports/
    phenome-application/
    phenome-adapter-primer/
    phenome-adapter-analytics/
    phenome-adapter-ml/
    phenome-ml/
  ui/
    phenome-ui-presentation/
    phenome-ui-core/
    phenome-ui-terminal/
    phenome-ui-tui/
```

## Dependency direction rules

Allowed edges:
- `ui-*` -> `ui-presentation` -> `application` -> `domain`
- `ui-tui` -> `domain` (status rendering and snapshot views)
- `ui-tui` -> `ports` (context wiring)
- `application` -> `ports`
- `adapters-*` -> `ports` + `domain`
- `ui-core` must not import `ratatui`, `crossterm`, or terminal helpers
- `ui-terminal` may call adapter CLI handlers; keep it TUI-free
- `bootstrappo` CLI must stay thin and defer to `phenome-ui-terminal`

Disallowed edges (examples):
- `domain` -> any interface, adapter, or presentation crate
- `ports` -> adapter or UI crates
- `adapters-*` -> `ui-*`

## Module-to-crate mapping (current state)

| Path | Target crate | Notes |
| --- | --- | --- |
| `lib/domain/phenome-domain/src/*` | `phenome-domain` | Core model types + snapshot state. |
| `lib/ports/phenome-ports/src/*` | `phenome-ports` | Action/Health/Log ports. |
| `lib/runtime/phenome-application/src/*` | `phenome-application` | Runtime orchestration. |
| `lib/runtime/phenome-ml/src/*` | `phenome-ml` | ML model implementations. |
| `lib/ui/phenome-ui-presentation/src/*` | `phenome-ui-presentation` | Shared formatting + logging config. |
| `lib/ui/phenome-ui-core/src/*` | `phenome-ui-core` | Framework-agnostic UI types. |
| `lib/ui/phenome-ui-terminal/src/*` | `phenome-ui-terminal` | CLI formatting + dispatch. |
| `lib/ui/phenome-ui-terminal/src/cli/*` | `phenome-ui-terminal` | Bootstrappo CLI clap surface. |
| `lib/ui/phenome-ui-tui/src/*` | `phenome-ui-tui` | Ratatui adapter and TUI logic. |
| `lib/adapters/phenome-adapter-primer/src/*` | `phenome-adapter-primer` | Port impls + bootstrappo mapping. |
| `lib/adapters/phenome-adapter-primer/src/controller/*` | `phenome-adapter-primer` | Bootstrappo command handlers. |
| `lib/adapters/phenome-adapter-analytics/src/*` | `phenome-adapter-analytics` | Analytics service components. |
| `lib/adapters/phenome-adapter-ml/src/*` | `phenome-adapter-ml` | ML service components. |
| `src/bin/cli.rs` | `cli` | Composition root for primer CLI. |
| `src/bin/tui.rs` | `tui` | Composition root for TUI. |
| `src/bin/analytics-service.rs` | `analytics-service` | Composition root for analytics service. |
| `src/bin/ml-service.rs` | `ml-service` | Composition root for ML service. |
| `src/lib.rs` | Workspace root or thin re-export | Prefer thin re-export only. |

## Decoupling candidates (non-tightly bound logic)

1) **BootstrappoBackend wiring**
   - Previous: `BootstrappoBackend` built `Runtime` inside adapters.
   - Now: adapter returns port set only; runtime built in bin crate.

2) **TUI App construction**
   - Previous: `phenome-ui-tui/app` depended on `BootstrappoBackend`.
   - Now: `App::new` accepts injected `Runtime` + `AppContext`.

3) **CLI formatting remains pure**
   - Keep formatters in `phenome-ui-terminal` limited to domain + presentation.
   - CLI dispatch lives alongside formatters and may call adapter handlers.

4) **UI-core independence**
   - Keep `phenome-ui-core` free of ratatui/crossterm and terminal deps.
   - Enforced by boundary tests.

## Implementation notes (initial)
- Composition roots now live in `src/bin/cli.rs` and `src/bin/tui.rs`.
- `BootstrappoBackend` exposes ports only; runtime construction happens in bins.
- TUI `start` accepts injected `Runtime` + `AppContext` instead of a backend.
- Domain types now live in `lib/domain/phenome-domain`.

## Migration checklist (ARCH-3C)
- [x] Add workspace members for target crates.
- [x] Scaffold empty crates under `lib/`.
- [x] Migrate core domain layer into `phenome-domain`.
- [x] Update imports to use `phenome_domain` crate paths.
- [x] Migrate ports into `phenome-ports`.
- [x] Migrate application runtime into `phenome-application`.
- [x] Migrate presentation helpers into `phenome-ui-presentation`.
- [x] Migrate UI core contracts into `phenome-ui-core`.
- [x] Migrate terminal formatting into `phenome-ui-terminal`.
- [x] Migrate TUI adapter into `phenome-ui-tui`.
- [x] Migrate bootstrappo adapter into `phenome-adapter-primer`.
- [x] Update bins to reference crate-level imports.

## Guardrails and tests

- Add crate boundary checks (deny higher-layer imports in lower crates).
- Keep `tests/interface_boundaries.rs` as a fast guardrail.
- Enforce crate dependency direction in `tests/crate_boundaries.rs`.
- Add CI tasks for `cargo check --no-default-features --features ui-core`.

## Open questions

- Do we want a dedicated `phenome-composition` crate for shared wiring
  used by both bins (instead of duplicating composition code)?
- Should `phenome-ui-presentation` be split into `formatting` vs `logging`
  sub-crates, or keep them together for stability?

## Next steps (post-ARCH-3)

1) Decide whether to add a shared composition crate for bin wiring.
2) Consider splitting `phenome-ui-presentation` into smaller sub-crates.
3) Tighten boundary tests as new adapters/interfaces are added.
