# ARCH-4 Module Responsibilities

## Purpose
Document the responsibilities of core and UI modules and align them with the
canonical layout in `docs/architecture/ARCH-4-structure.md`.

## Core Layer (lib/core)
- `rotappo-domain`: Domain models, enums, invariants, and identifiers.
- `rotappo-ports`: Port traits + contracts that adapters implement.
- `rotappo-application`: Runtime orchestration, pipelines, and state sync.
- `rotappo-adapter-bootstrappo`: Bootstrappo adapter implementations.
- `rotappo-adapter-analytics`: Analytics service adapter + schedulers.
- `rotappo-adapter-ml`: ML service adapter and inference hooks.
- `rotappo-adapter-notification`: Notification adapter.
- `rotappo-ml`: ML models + local inference helpers.

## UI Layer (lib/ui)
- `rotappo-ui-presentation`: UI-agnostic formatting, labels, and log helpers.
- `rotappo-ui-core`: Framework-agnostic UI contracts and types.
- `rotappo-ui-terminal`: CLI formatting + dispatch for bootstrappo CLI.
- `rotappo-ui-tui`: Ratatui adapter + TUI panels, layout, and bootstrap UI.

## Supporting Notes
- Each crate has a README at its root that defines ownership and boundaries.
- TUI adapter design principles are in
  `docs/architecture/ARCH-4-distributed-tui-design.md`.
