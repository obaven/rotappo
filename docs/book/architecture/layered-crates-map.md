# Layered Crates Map

The canonical map lives in `docs/architecture/ARCH-1A-layered-map.md`
and `docs/architecture/ARCH-4-structure.md`.
This chapter summarizes the current module tree:

```
lib/
  core/
    phenome-domain/
    phenome-ports/
    phenome-application/
    phenome-adapter-primer/
    phenome-adapter-analytics/
    phenome-adapter-ml/
    phenome-adapter-notification/
    phenome-ml/
  ui/
    phenome-ui-presentation/
    phenome-ui-core/
    phenome-ui-terminal/
    phenome-ui-tui/
src/
  bin/
    cli.rs
    tui.rs
    analytics-service.rs
    ml-service.rs
  lib.rs
```

Key rules:
- domain does not import adapters, presentation, or interfaces.
- ui-presentation does not import adapters or interfaces.
- ui interfaces only depend on application, ports, domain, and ui-presentation.
- `phenome-ui-terminal` may depend on the bootstrappo adapter for CLI dispatch.
