# ARCH-4 Module Responsibilities

## Purpose
Document the responsibilities of core and UI modules and align them with the
canonical layout in `docs/architecture/ARCH-4-structure.md`.

## Core Layer (lib/domain, lib/ports, lib/runtime, lib/adapters)
- `phenome-domain`: Domain models, enums, invariants, and identifiers.
- `phenome-ports`: Port traits + contracts that adapters implement.
- `phenome-application`: Runtime orchestration, pipelines, and state sync.
- `phenome-adapter-primer`: Bootstrappo adapter implementations.
- `phenome-adapter-analytics`: Analytics service adapter + schedulers.
- `phenome-adapter-ml`: ML service adapter and inference hooks.
- `phenome-adapter-notification`: Notification adapter.
- `phenome-ml`: ML models + local inference helpers.

## UI Layer (lib/ui)
- `phenome-ui-presentation`: UI-agnostic formatting, labels, and log helpers.
- `phenome-ui-core`: Framework-agnostic UI contracts and types.
- `phenome-ui-terminal`: CLI formatting + dispatch for primer CLI.
- `phenome-ui-tui`: Ratatui adapter + TUI panels, layout, and bootstrap UI.

## Supporting Notes
- Each crate has a README at its root that defines ownership and boundaries.
- TUI adapter design principles are in
  `docs/architecture/ARCH-4-distributed-tui-design.md`.
