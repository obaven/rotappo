# Presentation and Interfaces

Presentation code lives in `crates/ui/rotappo-ui-presentation` and contains shared formatting
and view-model helpers that are UI/CLI agnostic.

Interfaces live under `crates/ui/`:
- `crates/ui/rotappo-ui-terminal` for CLI formatting and output
- `crates/ui/rotappo-ui-tui` for TUI rendering, layout, and state
- `crates/ui/rotappo-ui-core` for framework-agnostic UI contracts

Interfaces should only render, handle input, and wire the application
layer together.

Feature flags:
- `ui-core` builds only the framework-agnostic UI contracts
- `cli` enables CLI formatters and the `terminal` binary
- `tui-ratatui` enables the TUI adapter and the `tui` binary

New UI surface checklist:
- Depend on `rotappo-ui-core` + `rotappo-ui-presentation` only (no TUI or terminal imports).
- Translate device events into `ui_core` input types at the adapter edge.
- Keep rendering and IO inside the adapter crate.
- Add a feature flag and update build docs.
- Extend boundary tests if new interface paths are introduced.
