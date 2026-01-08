# Migration and Guardrails

Reference documents:
- `docs/architecture/ARCH-1B-migration-action.md`
- `docs/architecture/ARCH-1C-guardrails.md`

Guardrails summary:
- Keep domain types pure and dependency-free.
- Keep ports domain-only and adapters translating external types.
- Keep interfaces thin and presentation-only.
- Treat the rotappo CLI surface as the sole source of truth for CLI behavior.
- Update call sites directly on any public path moves.
- Enforce crate dependency direction via `tests/crate_boundaries.rs`.
- Keep ui-core ratatui/terminal-free via `tests/interface_boundaries.rs`.
- Keep CLI boundary imports clean via `tests/cli_boundaries.rs`.
- Run CLI stability checks via `cargo make check-cli-stability`.
