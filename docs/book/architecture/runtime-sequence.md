# Runtime Data Flow

This sequence shows how data flows from adapters to interfaces.

```dot process
digraph RuntimeFlow {
  rankdir=TB;
  node [shape=box, style="rounded,filled", fillcolor="#f5f5f5"];

  adapter [label="Adapter (primer)\nassembly(), snapshot(), drain_events()"];
  ports [label="Ports\nAssemblyPort / HealthPort / LogPort"];
  application [label="Application Runtime\nbuild Snapshot + assembly steps"];
  presentation [label="Presentation\nformat assembly/problems/logs"];
  interfaces [label="Interfaces (CLI/TUI)\nrender / print"];

  adapter -> ports -> application -> presentation -> interfaces;
}
```

## Detailed steps

1) Adapter collects upstream data and normalizes it into domain types.
2) Ports expose the normalized data to the application layer.
3) Application builds a `Snapshot` and updates action status metadata.
4) Presentation formats the snapshot into view-friendly data.
5) Interfaces render output and handle user input.
