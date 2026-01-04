# ARCH-2 Interface Abstraction Plan

## Goal
Separate CLI- and ratatui-specific concerns from the interface layer so
future UI surfaces (web, desktop, etc.) can reuse UI contracts without
pulling CLI or ratatui dependencies.

## Current interface shape (summary)
- `crates/ui/rotappo-ui-tui/` is ratatui + crossterm heavy
  - runner/render/panels/layout/util depend on ratatui types
  - keyboard/input depends on crossterm
- `crates/ui/rotappo-ui-terminal/` is CLI formatting only
- `crates/ui/rotappo-ui-core/` defines framework-agnostic contracts
- No direct CLI use inside the TUI tree, but ratatui types are spread
  across most modules, making reuse by non-TUI surfaces difficult.

## Target module layout

Option A (explicit separation):

```
crates/ui/
  rotappo-ui-core/        (framework-agnostic UI contracts)
  rotappo-ui-tui/         (ratatui adapter)
    app/                  (tui state + handlers)
    layout/               (ratatui layout)
    panels/               (ratatui renderers)
    runner.rs             (crossterm loop)
  rotappo-ui-terminal/    (CLI formatting helpers)
```

Option B (nested core):

```
crates/ui/rotappo-ui-tui/
  core/           (ui-core contracts)
  ratatui/        (tui adapter implementation)
```

Recommendation: Option A. It keeps the UI-core contract stable and
allows additional adapters (web, desktop) without naming conflicts.

## Dependency rules
- `ui_core` may depend on `domain`, `application`, and `presentation`.
- `ui_core` must not depend on ratatui, crossterm, or CLI helpers.
- `tui` depends on `ui_core` and ratatui/crossterm.
- `terminal` remains independent; no `ui_core` imports required.

## Feature flags
- `ui-core`: builds framework-agnostic UI contracts only
- `tui-ratatui`: builds the TUI adapter (depends on `ui-core`)
- `cli`: builds CLI helpers and formatter output

Examples:
- `cargo check --no-default-features --features ui-core`
- `cargo check --no-default-features --features tui-ratatui`
- `cargo check --no-default-features --features cli`

## Guardrail tests
- Ensure `ui_core` compiles without ratatui/crossterm.
- Static check to prevent `ui_core` importing `rotappo-ui-terminal`.
- Build matrix tasks for `ui-core`, `tui-ratatui`, `cli` features.

## Migration steps (high level)
1) Extract UI-core contracts (state/events/models) from `rotappo-ui-tui`.
2) Move ratatui/crossterm code into `rotappo-ui-tui` adapter.
3) Add feature flags + build matrix tasks.
4) Add guardrail tests.
