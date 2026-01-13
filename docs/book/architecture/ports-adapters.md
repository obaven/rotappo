# Ports and Adapters

Ports define the runtime boundary for external data. They live in
`lib/core/rotappo-ports/` and use domain types only.

Adapters implement ports for concrete systems. The Bootstrappo adapter
lives in `lib/core/rotappo-adapter-bootstrappo/` and translates external types into
normalized domain types. Additional adapters live in:
- `lib/core/rotappo-adapter-analytics/`
- `lib/core/rotappo-adapter-ml/`
- `lib/core/rotappo-adapter-notification/`

Rules:
- UI/CLI do not import adapters directly, except `rotappo-ui-terminal` calling
  bootstrappo adapter command handlers for the bootstrappo CLI surface.
- Adapters do not import UI/CLI.
- Ports stay free of adapter-specific types.
