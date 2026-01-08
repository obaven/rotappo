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
- `cargo make check-cli-bootstrappo-surface`
- `cargo test --test interface_boundaries`
- `cargo test --test crate_boundaries`
- `cargo test --test cli_golden` (CLI output snapshots)
- `UPDATE_CLI_SNAPSHOTS=1 cargo test --test cli_golden` (update fixtures)
- `cargo check --features cli,module-bootstrappo --bin cli` (bootstrappo CLI)
- CLI output parity for action/problem formatting
- TUI smoke checks (header, action, logs panels)
