# Ports and Adapters

Ports define the runtime boundary for external data. They live in
`crates/core/rotappo-ports/` and use domain types only.

Adapters implement ports for concrete systems. The Bootstrappo adapter
lives in `crates/core/rotappo-adapter-bootstrappo/` and translates external types into
normalized domain types.

Rules:
- UI/CLI do not import adapters directly.
- Adapters do not import UI/CLI.
- Ports stay free of adapter-specific types.
