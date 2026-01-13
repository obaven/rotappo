# ARCH-2 Interface/UI Dependency Audit

## Scope
- `lib/ui/rotappo-ui-tui` (TUI implementation)
- `lib/ui/rotappo-ui-terminal` (CLI formatting + dispatch)
- `lib/ui/rotappo-ui-core` (framework-agnostic contracts)
- Cross-layer dependencies into domain/application/presentation

## Summary
- The TUI tree is ratatui + crossterm heavy and effectively acts as a
  concrete adapter, not a framework-agnostic UI core.
- No CLI-specific code is embedded inside the TUI tree today.
- A small subset of helpers are ratatui-free and can be promoted into
  a UI-core contract or shared presentation layer.
- `lib/ui/rotappo-ui-core/` now hosts the initial framework-agnostic
  contracts for future adapters.

## Dependency classification

Legend:
- UI-CORE: suitable for framework-agnostic reuse
- TUI-ADAPTER: ratatui/crossterm-specific implementation
- CLI: CLI-only formatting helpers

| Module/Path | Dependencies | Classification | Notes |
| --- | --- | --- | --- |
| `lib/ui/rotappo-ui-terminal/` | domain, presentation, adapter-bootstrappo | CLI | CLI formatting + dispatch. |
| `lib/ui/rotappo-ui-core/` | domain, presentation | UI-CORE | Framework-agnostic UI contracts. |
| `lib/ui/rotappo-ui-tui/runner.rs` | ratatui, crossterm | TUI-ADAPTER | TUI entrypoint wiring. |
| `lib/ui/rotappo-ui-tui/terminal.rs` | ratatui, crossterm | TUI-ADAPTER | Terminal guard + shared run loop. |
| `lib/ui/rotappo-ui-tui/render.rs` | ratatui | TUI-ADAPTER | Frame rendering pipeline. |
| `lib/ui/rotappo-ui-tui/panels/*` | ratatui | TUI-ADAPTER | Panel renderers. |
| `lib/ui/rotappo-ui-tui/layout/*` | ratatui | TUI-ADAPTER | Grid + layout resolver. |
| `lib/ui/rotappo-ui-tui/app/*` | ratatui, crossterm | TUI-ADAPTER | Input handling + UI behavior. |
| `lib/ui/rotappo-ui-tui/state/*` | ratatui | TUI-ADAPTER | Uses `ratatui::layout::Rect`. |
| `lib/ui/rotappo-ui-tui/util/time.rs` | domain | UI-CORE | Ratatui-free helper. |
| `lib/ui/rotappo-ui-tui/util/assembly.rs` | ratatui, presentation | TUI-ADAPTER | Assembly render helpers. |
| `lib/ui/rotappo-ui-tui/util/problems.rs` | presentation, app state | TUI-ADAPTER | Candidate for UI-core with App decoupling. |
| `lib/ui/rotappo-ui-tui/util/color.rs` | ratatui | TUI-ADAPTER | Ratatui color helpers. |
| `lib/ui/rotappo-ui-tui/util/rect.rs` | ratatui | TUI-ADAPTER | Ratatui layout utilities. |
| `lib/ui/rotappo-ui-tui/util/tooltip.rs` | ratatui | TUI-ADAPTER | Tooltip geometry. |

## CLI-specific sections inside `rotappo-ui-tui`
None detected. The CLI surface is isolated under `lib/ui/rotappo-ui-terminal`.
The refactor should keep CLI helpers separate and avoid importing them
into UI-core or TUI adapters.

## CLI-only UI behaviors
- Output mode selection (`OutputMode`), CLI formatters, and dispatch live in
  `lib/ui/rotappo-ui-terminal/`.
- The CLI formatters are currently exercised via golden tests; there is
  no standalone terminal binary.
- Shared view-model helpers remain in `lib/ui/rotappo-ui-presentation/formatting`
  and are UI/CLI neutral.

## Boundary note
- `rotappo-ui-core` and `rotappo-ui-tui` must not import `rotappo-ui-terminal`.
- If CLI-only UI helpers are added later, keep them under
  `lib/ui/rotappo-ui-terminal/` and avoid leaking them into `ui_core`.

## Proposed seams for UI-core extraction
- Extract ratatui-free helpers (`util/time.rs`) into `rotappo-ui-core`.
- Move UI state types into `ui_core` after replacing `ratatui::layout::Rect`
  with a neutral geometry type (or a lightweight internal struct).
- Convert `App` event handling to operate on UI-core input/event types,
  with ratatui/crossterm adapters translating device events.

## Risks
- UI-core extraction will require replacing ratatui types in state.
- Layout and panel rendering are tightly coupled to ratatui.
- Input handling currently depends directly on crossterm events.
