# ARCH-1C Guardrails + Caveats

This checklist keeps the refactor safe across UI, CLI, and runtime.

## Guardrail checklist
- Domain never imports `interfaces`, `presentation`, or `adapters`.
- Ports expose domain types only; adapters translate external types.
- UI depends on application + presentation only.
- CLI dispatch may call adapter handlers; keep it TUI-free.
- Shared formatting/logging lives under `rotappo-ui-presentation`.
- CLI boundaries: rotappo never imports bootstrappo CLI modules, and
  bootstrappo CLI never imports rotappo crates.
- Avoid compatibility shims; update call sites with moved paths.
- Update doc tests with new paths when public APIs move.
- Avoid circular dependencies (adapters -> ports -> domain only).
- Interface boundaries: `ui_core` stays ratatui/terminal-free (see
  `docs/book/architecture/presentation-interfaces.md`).

## Known risks + mitigations
- Serialization drift in domain types: keep adapter mapping tests and
  versioned snapshots where applicable.
- CLI output parity regressions: add golden-output or snapshot checks.
- Runtime behavior drift after port normalization: compare snapshots
  before/after changes.
- TUI rendering regressions: smoke test header, action, logs panels.
- Adapter types leaking into ports: review imports and enforce linting.

## Suggested validation per PR
- `cargo test -p rotappo` (or workspace equivalent).
- `cargo run --features cli,module-bootstrappo --bin cli -- --help` for CLI surface sanity.
- Manual TUI smoke check (header, action, logs panels).
- Doc tests updated alongside any public path changes.
- `cargo test --test interface_boundaries` to enforce interface layering.
- `cargo test --test crate_boundaries` to enforce crate dependency direction.
- `cargo test --test cli_boundaries` to enforce CLI boundary imports.
