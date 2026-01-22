# ARCH-2 Interface Abstraction Action

## Goal
Separate CLI- and ratatui-specific concerns from the interface layer so
future UI surfaces (web, desktop, etc.) can reuse UI contracts without
pulling CLI or ratatui dependencies.

## Current interface shape (summary)
- `lib/ui/phenome-ui-tui/` is ratatui + crossterm heavy
  - runner/render/panels/layout/util depend on ratatui types
  - keyboard/input depends on crossterm
- `lib/ui/phenome-ui-terminal/` is CLI formatting only
- `lib/ui/phenome-ui-core/` defines framework-agnostic contracts
- No direct CLI use inside the TUI tree, but ratatui types are spread
  across most modules, making reuse by non-TUI surfaces difficult.

## Target module layout

Option A (explicit separation):

```
lib/ui/
  phenome-ui-core/        (framework-agnostic UI contracts)
  phenome-ui-tui/         (ratatui adapter)
    app/                  (tui state + handlers)
    layout/               (ratatui layout)
    panels/               (ratatui renderers)
    runner.rs             (crossterm loop)
  phenome-ui-terminal/    (CLI formatting helpers)
```

Option B (nested core):

```
lib/ui/phenome-ui-tui/
  core/           (ui-core contracts)
  ratatui/        (tui adapter implementation)
```

Recommendation: Option A. It keeps the UI-core contract stable and
allows additional adapters (web, desktop) without naming conflicts.

## Dependency rules
- `ui_core` may depend on `domain`, `application`, and `presentation`.
- `ui_core` must not depend on ratatui, crossterm, or CLI helpers.
- `tui` depends on `ui_core` and ratatui/crossterm.
- CLI formatting helpers remain independent; no `ui_core` imports required.

## Feature flags
- `ui-core`: builds framework-agnostic UI contracts only
- `tui-ratatui`: builds the TUI adapter (depends on `ui-core`; requires a module feature)
- `cli`: builds CLI helpers and formatter output (requires a module feature)
- `module-primer`: enables the primer CLI surface (with `cli`)
- `module-plasmid`: enables the plasmid CLI surface (with `cli`)

Examples:
- `cargo check --no-default-features --features ui-core`
- `cargo check --no-default-features --features tui-ratatui,module-primer`
- `cargo check --no-default-features --features cli,module-primer`

## Guardrail tests
- Ensure `ui_core` compiles without ratatui/crossterm.
- Static check to prevent `ui_core` importing `phenome-ui-terminal`.
- Build matrix tasks for `ui-core`, `tui-ratatui + module-primer`, `cli + module-primer` features.

## Migration steps (high level)
1) Extract UI-core contracts (state/events/models) from `phenome-ui-tui`.
2) Move ratatui/crossterm code into `phenome-ui-tui` adapter.
3) Add feature flags + build matrix tasks.
4) Add guardrail tests.
