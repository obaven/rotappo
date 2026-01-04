# Terminal (CLI)

CLI formatting helpers live in `crates/ui/rotappo-ui-terminal/`. They expose
formatters for snapshots, plans, events, actions, and problems.

Output modes:
- plain
- json
- ndjson

The CLI binary is `src/bin/terminal.rs`.

Build:
- `cargo run --bin terminal --features cli`
