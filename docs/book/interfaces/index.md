# Interfaces

Interfaces are the terminal (CLI) and TUI entry points under
`crates/ui/`. They should not carry domain logic, only rendering,
I/O, and wiring.

Common helpers live in `crates/ui/rotappo-ui-presentation/`.
