# ADR-0001: Canonical Module Layout and Responsibilities

## Status
Accepted (2026-01-06)

## Context
The repository accumulated multiple layouts and ambiguous module ownership.
This created drift in docs, unclear adapter boundaries, and repeated
refactors when adding new tools or UI surfaces.

## Decision
Adopt the ARCH-4 canonical layout and module responsibility map:
- `docs/architecture/ARCH-4-structure.md` is the canonical layout source.
- `docs/architecture/ARCH-4-module-responsibilities.md` defines ownership.
- Each crate must maintain a README documenting its boundary and dependencies.

## Consequences
- New modules must follow the canonical layout and update the module map.
- Docs and runbooks must reference the canonical paths.
- Boundary tests remain the enforcement mechanism for layer direction.

## Alternatives considered
- Keep the existing layout and document exceptions (rejected due to drift).
- Split core/UI into separate repositories (rejected due to integration cost).

## Links
- `docs/architecture/ARCH-4-structure.md`
- `docs/architecture/ARCH-4-module-responsibilities.md`
- `tests/crate_boundaries.rs`
