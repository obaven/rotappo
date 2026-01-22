# ARCH-4 Macro + Helper Guide

## Purpose
Inventory macros and shared helpers, document ownership, and set rules for
macro usage so the distributed TUI adapter stays modular and layered.

## Macro Inventory
TUI adapter macros (owner: `phenome-ui-tui`):
- `grid_spec!` (grid spec builder)
- `grid_slot!` (slot builder with optional flags)
- `grid_slots!` (slot list builder)
- `grid_slot_opts!` (slot option chain)

Source of record: `lib/ui/phenome-ui-tui/src/macros/layout.rs`.

## Helper Inventory + Duplication Hotspots
- `centered_rect` existed in `bootstrap/utils.rs` and `util/rect.rs`. It now
  lives in `lib/ui/phenome-ui-tui/src/util/rect.rs`, and bootstrap panels
  consume the shared helper.
- No core-layer macros are defined today, and no other helper duplication
  hotspots were identified in `lib/ui/phenome-ui-tui/src/util`.

## Usage Guidance
- Prefer `phenome_ui_tui::grid_spec!` or `crate::grid_spec!` for layout grids.
- Use `phenome_ui_tui::util` helpers for Ratatui geometry and rendering glue.
- Keep macro usage limited to layout and rendering scaffolding; do not embed
  domain rules or cross-adapter assumptions.

## Stability + Layering Rules
- Macros live in the layer that owns the types they compose.
- Macros must not import or reference deeper layers (domain/ports/application).
- Shared macros across layers require an ARCH-4B update and documentation.
- Macro updates require this inventory to be updated alongside call sites.
