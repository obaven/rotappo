# Interfaces

Interfaces include the TUI entry point under `crates/ui/` and the
bootstrappo CLI under `src/bin/cli.rs`. This CLI is the
sole source of truth for bootstrappo CLI behavior. CLI parsing and dispatch
live in `crates/ui/rotappo-ui-terminal`, while command handlers live in the
bootstrappo adapter.

Common helpers live in `crates/ui/rotappo-ui-presentation/`.
