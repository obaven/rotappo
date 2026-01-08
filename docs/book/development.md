# Development Workflow

Recommended tools:
- cargo-make for task orchestration
- mdbook for documentation builds

Install tools:

```
cargo install cargo-make mdbook
```

Graphviz rendering (optional):

```
cargo install mdbook-graphviz
```

Graphviz preprocessor is enabled in `docs/book/book.toml`.

Version compatibility:

```
mdbook --version
# mdbook 0.4.x  -> mdbook-graphviz 0.2.x
# mdbook 0.5.x  -> mdbook-graphviz 0.3.x
```

Example (mdbook 0.4.x):

```
cargo install mdbook-graphviz --version 0.2.1 --force
```

Common tasks (via cargo-make):

```
cargo make check
cargo make test
cargo make doc
cargo make docbook
```

Bootstrappo CLI (rotappo, source of truth):

```
cargo run --features cli,module-bootstrappo --bin cli -- --help
```

Guardrail checks:

```
cargo make check-cli-stability
cargo make check-cli-bootstrappo-surface
cargo make check-interfaces
cargo make check-boundaries
cargo make check-guardrails
```

CI should run `cargo make check-cli-stability` (or `cargo make check-guardrails`) to gate CLI drift.

CLI snapshot updates:

```
UPDATE_CLI_SNAPSHOTS=1 cargo test --test cli_golden
```

Aliases:

```
cargo docbook
cargo dockbook
```

See `Makefile.toml` for the full task list.
