# Migration and Guardrails

Reference documents:
- `docs/architecture/ARCH-1B-migration-plan.md`
- `docs/architecture/ARCH-1C-guardrails.md`

Guardrails summary:
- Keep domain types pure and dependency-free.
- Keep ports domain-only and adapters translating external types.
- Keep interfaces thin and presentation-only.
- Update call sites directly on any public path moves.
- Enforce crate dependency direction via `tests/crate_boundaries.rs`.
- Keep ui-core ratatui/terminal-free via `tests/interface_boundaries.rs`.
