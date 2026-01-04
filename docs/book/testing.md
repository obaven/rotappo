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
- `cargo test --test interface_boundaries`
- `cargo test --test crate_boundaries`
- CLI output parity for plan/problem formatting
- TUI smoke checks (header, plan, logs panels)
