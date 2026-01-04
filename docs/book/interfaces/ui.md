# TUI

TUI code lives in `crates/ui/rotappo-ui-tui/` and is split into:
- app: application state and interactions
- layout: grid layout, policy, and shell specs
- panels: renderers for each UI panel
- state: hover, tooltips, and log menu state
- util: shared rendering helpers

The TUI binary is `src/bin/tui.rs`.

Build:
- `cargo run --bin tui --features tui-ratatui`
