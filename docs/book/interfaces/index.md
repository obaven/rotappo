# Interfaces

Interfaces include the TUI entry point under `lib/ui/` and the
primer CLI under `src/bin/cli.rs`. This CLI is the
sole source of truth for primer CLI behavior. CLI parsing and dispatch
live in `lib/ui/phenome-ui-terminal`, while command handlers live in the
bootstrappo adapter.

Common helpers live in `lib/ui/phenome-ui-presentation/`.
Canonical layout and responsibilities live in:
- `docs/architecture/ARCH-4-structure.md`
- `docs/architecture/ARCH-4-module-responsibilities.md`
