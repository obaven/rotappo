# ARCH-2 UI Core Contracts

## Purpose
UI core contracts define view state, input events, and UI intents in a
framework-agnostic way. They allow new UI surfaces (web, desktop) to
reuse the same core types without ratatui/crossterm dependencies.

## Module location
- `lib/ui/phenome-ui-core/`

## Core types

### Geometry
- `UiPoint` (x, y)
- `UiRect` (x, y, width, height)
- `UiMargin` (horizontal, vertical)

### Input events
- `UiInputEvent` (Key, Mouse, Tick)
- `UiKeyEvent` / `UiKeyCode` / `UiKeyState` / `UiKeyModifiers`
- `UiMouseEvent` / `UiMouseButton` / `UiMouseKind`

### UI intents
- `UiIntent` (high-level intent derived from input)

### View state
- `UiLayoutState` (rectangles for panel areas)
- `UiViewState` (hover indices, collapse flags, scroll positions,
  log config, and cached events)
- `UiTooltip`, `UiHoldState`, `UiHoverPanel`, `UiLogMenuMode`

## Dependency rules
- `ui_core` may depend on `domain` and `presentation` only.
- `ui_core` must not import ratatui or crossterm.
- `ui_core` must not import `phenome-ui-terminal`.

## Adapter mapping
- TUI adapter translates crossterm events to `UiInputEvent`.
- Ratatui layouts are mapped to `UiLayoutState` rectangles.
- UI intents produced by adapters feed application-level actions.

## Next steps
- Move ratatui types out of `phenome-ui-tui/state` into `ui_core`.
- Add adapter mapping helpers in the TUI layer.
- Add feature flags so `ui_core` compiles without TUI dependencies.
