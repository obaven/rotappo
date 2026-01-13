# ARCH-2 TUI Adapter

## Purpose
Isolate ratatui/crossterm-specific implementation behind a dedicated
TUI adapter module so non-TUI surfaces can reuse `ui_core` without
pulling terminal UI dependencies.

## Module location
- `lib/ui/rotappo-ui-tui/`

## Contents
- `runner.rs`: TUI entrypoint wiring
- `terminal.rs`: terminal guard + shared run loop (crossterm)
- `render.rs`: frame rendering pipeline (ratatui)
- `panels/`: main TUI panels (ratatui)
- `bootstrap/`: bootstrap-only TUI panels and state
- `layout/`: grid layout + resolver + shell specs (ratatui)
- `app/`: TUI behavior + input handlers
- `state/`: TUI state structs (ratatui types remain here for now)

## Dependency rules
- `rotappo-ui-tui` may depend on `rotappo-ui-core`, `rotappo-ui-presentation`, `rotappo-application`.
- `rotappo-ui-tui` may use ratatui/crossterm.
- `rotappo-ui-tui` must not import `rotappo-ui-terminal`.

## Public entrypoint
- `src/bin/tui.rs` calls `rotappo_ui_tui::start(runtime, context)`.

## Next steps
- Map crossterm input into `ui_core::UiInputEvent`.
- Replace ratatui `Rect` in state with `ui_core::UiRect`.
- Add feature flag to compile TUI only when requested.
- See `docs/architecture/ARCH-4-distributed-tui-design.md` for
  distributed TUI design principles.
- Decision record: `docs/architecture/adr/0002-distributed-tui-adapter.md`.
