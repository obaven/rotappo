# Testing and Validation

Core test commands:

```
cargo test
cargo test --lib
cargo test --bins
```

Recommended checks:
- `cargo fmt --check`
- `cargo clippy --all-targets`
- `cargo make check-cli-stability`
- `cargo make check-cli-primer-surface`
- `cargo test --test interface_boundaries`
- `cargo test --test crate_boundaries`
- `cargo test --test cli_golden` (CLI output snapshots)
- `UPDATE_CLI_SNAPSHOTS=1 cargo test --test cli_golden` (update fixtures)
- `cargo check --features cli,module-primer --bin cli` (primer CLI)
- CLI output parity for action/problem formatting
- TUI smoke checks (navbar navigation, Analytics Real-time view renders, Topology Assembly view renders, Terminal Log Stream view updates, notifications overlay toggles)

Cleanup verification (ARCH-4C):
- Analytics service loops respect shutdown and tick budgets (metrics, retention, scheduler, anomaly watcher)
- Scheduler loop caps per-tick executions and logs when deferring work
- Notification anomaly loop caps per tick and logs errors on query/send
- TUI analytics background updates are capped to avoid UI starvation
- Bootstrappo live status loop has an explicit stop path
