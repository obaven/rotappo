# ARCH-2 TUI Adapter

## Purpose
Isolate ratatui/crossterm-specific implementation behind a dedicated
TUI adapter module so non-TUI surfaces can reuse `ui_core` without
pulling terminal UI dependencies.

## Module location
- `crates/ui/rotappo-ui-tui/`

## Contents
- `runner.rs`: terminal setup and event loop (crossterm)
- `render.rs`: frame rendering pipeline (ratatui)
- `panels/`: panel renderers (ratatui)
- `layout/`: grid layout + resolver (ratatui)
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
