# rotappo-ui-terminal

Layer: interfaces

Purpose:
- CLI formatting and dispatch wiring for the bootstrappo surface.

Dependencies:
- rotappo-ui-presentation
- rotappo-application
- rotappo-ports

Boundaries:
- No ratatui dependencies.
- CLI dispatch may call bootstrappo adapter handlers.
