# TUI

TUI code lives in `lib/ui/phenome-ui-tui/` and is split into:
- app: application state and interactions
- analytics_client: analytics gRPC client + cache updates
- bootstrap: bootstrappo bootstrap UI panels and state
- layout: grid layout and shell specs
- panels: renderers for each UI panel
- state: hover, tooltips, and UI state
- util: shared rendering helpers
- macros: layout macros for grid specs and slots
- render/runner/terminal: render pipeline + shared loop + entrypoint wiring

The TUI binary is `src/bin/tui.rs`.

Navigation is navbar-based with 3 sections:
- Analytics: Real-time, Historical, Predictions, Recommendations, Insights
- Topology: Assembly Steps, Domains, Capabilities, Queue State, Health, DAG Graph, Dual Graph
- Terminal: Log Stream, Event Feed, Commands, Diagnostics

Shell panels rendered in every view:
- navbar, main view, footer help, notifications overlay
- confirmation and tooltip overlays (contextual)

Collapse state lives in `UiState` and is toggled explicitly by input handlers;
the layout system does not auto-collapse panels on resize.

Build:
- `cargo run --bin tui --features tui,module-primer`

System design principles:
- `docs/architecture/ARCH-4-distributed-tui-design.md`
