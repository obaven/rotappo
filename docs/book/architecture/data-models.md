# Data Models

This section documents the core domain types used by the application,
presentation, and interface layers.

## Relationship map

```dot process
digraph DomainModel {
  rankdir=LR;
  node [shape=box, style="rounded,filled", fillcolor="#f5f5f5"];

  Snapshot -> PlanSummary;
  Snapshot -> PlanStep [label="plan_steps*"];
  Snapshot -> Capability [label="capabilities*"];
  Snapshot -> ActionId [label="last_action?"];
  Snapshot -> ActionStatus [label="last_action_status?"];

  PlanStep -> PlanStepStatus;
  Capability -> CapabilityStatus;

  Action -> ActionId;
  Action -> ActionSafety;

  Event -> EventLevel;

  HealthSnapshot -> ComponentHealthStatus [label="health map"];

  Plan -> PlanStepDef [label="steps*"];
}
```

## Snapshot

`Snapshot` is the aggregated runtime view.

| Field | Type | Notes |
| --- | --- | --- |
| `plan` | `PlanSummary` | Plan counters and totals. |
| `plan_steps` | `Vec<PlanStep>` | Materialized plan steps. |
| `capabilities` | `Vec<Capability>` | Capability status list. |
| `health` | `HealthStatus` | Coarse health summary. |
| `last_updated_ms` | `u64` | Millisecond timestamp. |
| `last_action` | `Option<ActionId>` | Most recent action id. |
| `last_action_status` | `Option<ActionStatus>` | Status for last action. |

## PlanSummary

| Field | Type | Notes |
| --- | --- | --- |
| `total` | `u32` | Step count. |
| `completed` | `u32` | Steps completed. |
| `in_progress` | `u32` | Steps running. |
| `blocked` | `u32` | Steps blocked. |
| `pending` | `u32` | Steps pending. |

## PlanStep

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `String` | Unique step id. |
| `kind` | `String` | Step kind (apply, delete, etc.). |
| `depends_on` | `Vec<String>` | Step dependencies. |
| `provides` | `Vec<String>` | Outputs from the step. |
| `status` | `PlanStepStatus` | Pending, Running, Succeeded, Failed, Blocked. |
| `domain` | `String` | Domain grouping. |
| `pod` | `Option<String>` | Optional execution unit. |

## Capability

| Field | Type | Notes |
| --- | --- | --- |
| `name` | `String` | Capability name. |
| `status` | `CapabilityStatus` | Ready, Degraded, Offline. |

## HealthSnapshot

`HealthSnapshot` normalizes adapter health signals.

| Field | Type | Notes |
| --- | --- | --- |
| `health` | `HashMap<String, ComponentHealthStatus>` | Component status map. |
| `last_error` | `Option<String>` | Most recent error. |
| `cache_ready` | `bool` | Whether cache is ready. |

## Plan (definitions)

`Plan` is the adapter-normalized plan used by the application layer.

| Field | Type | Notes |
| --- | --- | --- |
| `steps` | `Vec<PlanStepDef>` | Plan step definitions. |

`PlanStepDef` fields:

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `String` | Unique step id. |
| `kind` | `String` | Step kind. |
| `depends_on` | `Vec<String>` | Dependencies. |
| `provides` | `Vec<String>` | Outputs. |
| `domain` | `String` | Domain grouping. |
| `pod` | `Option<String>` | Optional execution unit. |
| `has_gates` | `bool` | Gate presence. |

## Event

| Field | Type | Notes |
| --- | --- | --- |
| `timestamp_ms` | `u64` | Millisecond timestamp. |
| `level` | `EventLevel` | Info, Warn, Error. |
| `message` | `String` | Log message. |

## Action

| Field | Type | Notes |
| --- | --- | --- |
| `id` | `ActionId` | Validate, Reconcile, Rotate, Nuke, Debug. |
| `label` | `&'static str` | Display label. |
| `description` | `&'static str` | Human description. |
| `requires_confirmation` | `bool` | Whether confirmation is required. |
| `safety` | `ActionSafety` | Safe, Guarded, Destructive. |
