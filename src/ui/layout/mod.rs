pub mod dsl;
pub mod shell_spec;
pub mod grid;
pub mod policy;
pub mod renderer;

pub use grid::{
    GridCache, GridLayout, GridResolver, GridSlot, GridSpec, SlotId, SpinLock,
    TrackSize,
};
pub use policy::{LayoutPolicy, PanelPriority, SlotPolicy, SlotOverride};
pub use renderer::LayoutRenderer;
pub use shell_spec::{
    footer_spec, left_column_spec, middle_column_spec, right_columns_spec,
    plan_header_spec, right_left_spec, right_right_spec, tui_shell_spec,
    tui_shell_spec_with_footer, SLOT_ACTIONS, SLOT_AUX, SLOT_BODY,
    SLOT_CAPABILITIES, SLOT_FOOTER, SLOT_FOOTER_HELP, SLOT_FOOTER_SETTINGS,
    SLOT_HEADER, SLOT_LEFT, SLOT_LOGS, SLOT_LOG_CONTROLS, SLOT_MIDDLE,
    SLOT_PLAN, SLOT_PLAN_PROGRESS, SLOT_PLAN_STEPS, SLOT_PROBLEMS, SLOT_RIGHT,
    SLOT_RIGHT_LEFT, SLOT_RIGHT_RIGHT, SLOT_SNAPSHOT,
};
