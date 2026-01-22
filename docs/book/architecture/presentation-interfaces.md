# Presentation and Interfaces

Presentation code lives in `lib/ui/phenome-ui-presentation` and contains shared formatting
and view-model helpers that are UI/CLI agnostic.

Interfaces live under `lib/ui/`:
- `lib/ui/phenome-ui-terminal` for CLI formatting and dispatch
- `lib/ui/phenome-ui-tui` for TUI rendering, layout, and state
- `lib/ui/phenome-ui-core` for framework-agnostic UI contracts

Interfaces should only render, handle input, and wire the application
layer together. The primer CLI dispatch in `phenome-ui-terminal`
may call adapter command handlers.

Feature flags:
- `ui-core` builds only the framework-agnostic UI contracts
- `cli` enables CLI formatters and the `cli` binary (requires a module feature)
- `tui-ratatui` enables the TUI adapter and the `tui` binary (requires a module feature)
- `module-primer` enables the primer CLI surface (with `cli`)
- `module-plasmid` enables the plasmid CLI surface (with `cli`)

Bootstrappo CLI:
- `src/bin/cli.rs` (enabled via `cli` + `module-primer`)

New UI surface checklist:
- Depend on `phenome-ui-core` + `phenome-ui-presentation` only (no TUI or terminal imports).
- Translate device events into `ui_core` input types at the adapter edge.
- Keep rendering and IO inside the adapter crate.
- Add a feature flag and update build docs.
- Extend boundary tests if new interface paths are introduced.
