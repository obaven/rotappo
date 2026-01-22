# Terminal (CLI)

CLI formatting helpers live in `lib/ui/phenome-ui-terminal/`. They expose
formatters for snapshots, actions, events, actions, and problems. There is no
standalone `terminal` binary; the primer CLI lives under
`src/bin/cli.rs` and is wired through `phenome-ui-terminal`. This
CLI is the sole source of truth for primer CLI behavior.

Output modes:
- plain
- json
- ndjson

Bootstrappo CLI:
- `cargo run --features cli,module-primer --bin cli -- --help`

Runbook:
- `docs/book/runbooks/cli.md`
