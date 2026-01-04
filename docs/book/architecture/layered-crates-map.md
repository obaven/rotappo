# Layered Crates Map

The canonical map lives in `docs/architecture/ARCH-1A-layered-map.md`.
This chapter summarizes the current module tree:

```
crates/
  core/
    rotappo-domain/
    rotappo-ports/
    rotappo-application/
    rotappo-adapter-bootstrappo/
  ui/
    rotappo-ui-presentation/
    rotappo-ui-core/
    rotappo-ui-terminal/
    rotappo-ui-tui/
src/
  bin/
  lib.rs
```

Key rules:
- domain does not import adapters, presentation, or interfaces.
- ui-presentation does not import adapters or interfaces.
- ui interfaces only depend on application, ports, domain, and ui-presentation.
