# Terminal (CLI)

CLI formatting helpers live in `crates/ui/rotappo-ui-terminal/`. They expose
formatters for snapshots, actions, events, actions, and problems. There is no
standalone `terminal` binary; the bootstrappo CLI lives under
`src/bin/cli.rs` and is wired through `rotappo-ui-terminal`. This
CLI is the sole source of truth for bootstrappo CLI behavior.

Output modes:
- plain
- json
- ndjson

Bootstrappo CLI:
- `cargo run --features cli,module-bootstrappo --bin cli -- --help`

Runbook:
- `docs/book/runbooks/cli.md`
