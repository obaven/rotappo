use ratatui::layout::Rect;

use crate::ui::layout::{GridResolver, GridSpec, TrackSize};

pub const SLOT_HEADER: &str = "header";
pub const SLOT_BODY: &str = "body";
pub const SLOT_LEFT: &str = "left";
pub const SLOT_MIDDLE: &str = "middle";
pub const SLOT_RIGHT: &str = "right";
pub const SLOT_FOOTER: &str = "footer";
pub const SLOT_PLAN: &str = "plan";
pub const SLOT_PLAN_PROGRESS: &str = "plan_progress";
pub const SLOT_SNAPSHOT: &str = "snapshot";
pub const SLOT_CAPABILITIES: &str = "capabilities";
pub const SLOT_PLAN_STEPS: &str = "plan_steps";
pub const SLOT_AUX: &str = "aux";
pub const SLOT_RIGHT_LEFT: &str = "right_left";
pub const SLOT_RIGHT_RIGHT: &str = "right_right";
pub const SLOT_ACTIONS: &str = "actions";
pub const SLOT_PROBLEMS: &str = "problems";
pub const SLOT_LOG_CONTROLS: &str = "log_controls";
pub const SLOT_LOGS: &str = "logs";
pub const SLOT_FOOTER_HELP: &str = "footer_help";
pub const SLOT_FOOTER_SETTINGS: &str = "footer_settings";

pub fn tui_shell_spec() -> GridSpec {
    tui_shell_spec_with_footer(4)
}

pub fn tui_shell_spec_with_footer(footer_height: u16) -> GridSpec {
    crate::grid_spec!(
        rows: [
            TrackSize::Fixed(3),
            TrackSize::Fill(1),
            TrackSize::Fixed(footer_height.max(2)),
        ],
        cols: [
            TrackSize::Percent(30),
            TrackSize::Percent(35),
            TrackSize::Percent(35),
        ],
        slots: [
            crate::grid_slot!(SLOT_HEADER, 0, 0, span: (1, 3), min: (20, 3)),
            crate::grid_slot!(SLOT_BODY, 1, 0, span: (1, 3), min: (20, 8)),
            crate::grid_slot!(SLOT_LEFT, 1, 0, min: (12, 8)),
            crate::grid_slot!(SLOT_MIDDLE, 1, 1, min: (16, 8)),
            crate::grid_slot!(SLOT_RIGHT, 1, 2, min: (16, 8)),
            crate::grid_slot!(SLOT_FOOTER, 2, 0, span: (1, 3), min: (20, 4)),
        ]
    )
}

pub fn left_column_spec(
    plan_progress_collapsed: bool,
    snapshot_collapsed: bool,
    collapsed_capabilities: bool,
    collapsed_height: u16,
) -> GridSpec {
    let plan_progress_height = if plan_progress_collapsed {
        collapsed_height
    } else {
        3
    };
    let snapshot_height = if snapshot_collapsed { collapsed_height } else { 4 };
    let left_top_height = plan_progress_height.saturating_add(snapshot_height);
    let top = if collapsed_capabilities {
        TrackSize::Min(left_top_height)
    } else {
        TrackSize::Fixed(left_top_height)
    };
    let bottom = if collapsed_capabilities {
        TrackSize::Fixed(collapsed_height)
    } else {
        TrackSize::Fill(1)
    };
    crate::grid_spec!(
        rows: [top, bottom],
        cols: [TrackSize::Fill(1)],
        slots: [
            crate::grid_slot!(SLOT_PLAN, 0, 0, min: (12, 4)),
            crate::grid_slot!(SLOT_CAPABILITIES, 1, 0, min: (12, 4)),
        ]
    )
}

pub fn plan_header_spec(
    plan_progress_collapsed: bool,
    snapshot_collapsed: bool,
    collapsed_height: u16,
) -> GridSpec {
    let plan_progress_track = if plan_progress_collapsed {
        TrackSize::Fixed(collapsed_height)
    } else if snapshot_collapsed {
        TrackSize::Min(3)
    } else {
        TrackSize::Fixed(3)
    };
    let snapshot_track = if snapshot_collapsed {
        TrackSize::Fixed(collapsed_height)
    } else {
        TrackSize::Min(4)
    };
    crate::grid_spec!(
        rows: [plan_progress_track, snapshot_track],
        cols: [TrackSize::Fill(1)],
        slots: [
            crate::grid_slot!(SLOT_PLAN_PROGRESS, 0, 0, min: (12, 3)),
            crate::grid_slot!(SLOT_SNAPSHOT, 1, 0, min: (12, 4)),
        ]
    )
}

pub fn middle_column_spec(plan_steps_collapsed: bool, collapsed_height: u16) -> GridSpec {
    if plan_steps_collapsed {
        crate::grid_spec!(
            rows: [TrackSize::Fill(1), TrackSize::Fixed(collapsed_height)],
            cols: [TrackSize::Fill(1)],
            slots: [
                crate::grid_slot!(SLOT_AUX, 0, 0, min: (16, 4)),
                crate::grid_slot!(SLOT_PLAN_STEPS, 1, 0, min: (16, 2)),
            ]
        )
    } else {
        crate::grid_spec!(
            rows: [TrackSize::Fill(1)],
            cols: [TrackSize::Fill(1)],
            slots: [crate::grid_slot!(SLOT_PLAN_STEPS, 0, 0, min: (16, 8))]
        )
    }
}

pub fn right_columns_spec() -> GridSpec {
    crate::grid_spec!(
        rows: [TrackSize::Fill(1)],
        cols: [TrackSize::Percent(45), TrackSize::Percent(55)],
        slots: [
            crate::grid_slot!(SLOT_RIGHT_LEFT, 0, 0, min: (12, 8)),
            crate::grid_slot!(SLOT_RIGHT_RIGHT, 0, 1, min: (12, 8)),
        ]
    )
}

pub fn right_left_spec(
    collapsed_actions: bool,
    collapsed_problems: bool,
    collapsed_height: u16,
) -> GridSpec {
    let (actions_track, problems_track) = match (collapsed_actions, collapsed_problems) {
        (true, true) => (
            TrackSize::Fixed(collapsed_height),
            TrackSize::Fixed(collapsed_height),
        ),
        (true, false) => (TrackSize::Fixed(collapsed_height), TrackSize::Fill(1)),
        (false, true) => (TrackSize::Fill(1), TrackSize::Fixed(collapsed_height)),
        (false, false) => (TrackSize::Min(8), TrackSize::Min(4)),
    };
    crate::grid_spec!(
        rows: [actions_track, problems_track],
        cols: [TrackSize::Fill(1)],
        slots: [
            crate::grid_slot!(SLOT_ACTIONS, 0, 0, min: (12, 4)),
            crate::grid_slot!(SLOT_PROBLEMS, 1, 0, min: (12, 4)),
        ]
    )
}

pub fn right_right_spec(
    log_controls_height: u16,
    collapsed_log_controls: bool,
    collapsed_logs: bool,
    collapsed_height: u16,
) -> GridSpec {
    let (controls_track, logs_track) = match (collapsed_log_controls, collapsed_logs) {
        (true, true) => (
            TrackSize::Fixed(collapsed_height),
            TrackSize::Fixed(collapsed_height),
        ),
        (true, false) => (TrackSize::Fixed(collapsed_height), TrackSize::Fill(1)),
        (false, true) => (TrackSize::Fill(1), TrackSize::Fixed(collapsed_height)),
        (false, false) => (TrackSize::Fixed(log_controls_height), TrackSize::Min(6)),
    };
    crate::grid_spec!(
        rows: [controls_track, logs_track],
        cols: [TrackSize::Fill(1)],
        slots: [
            crate::grid_slot!(SLOT_LOG_CONTROLS, 0, 0, min: (12, 3)),
            crate::grid_slot!(SLOT_LOGS, 1, 0, min: (12, 6)),
        ]
    )
}

pub fn footer_spec() -> GridSpec {
    crate::grid_spec!(
        rows: [TrackSize::Fill(1)],
        cols: [TrackSize::Percent(35), TrackSize::Percent(65)],
        slots: [
            crate::grid_slot!(SLOT_FOOTER_HELP, 0, 0, min: (12, 2)),
            crate::grid_slot!(SLOT_FOOTER_SETTINGS, 0, 1, min: (12, 2)),
        ]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_shell_spec() {
        let spec = tui_shell_spec();
        let layout = GridResolver::resolve(Rect::new(0, 0, 120, 40), &spec);
        let header = layout.rect(SLOT_HEADER).expect("header");
        let footer = layout.rect(SLOT_FOOTER).expect("footer");
        let body = layout.rect(SLOT_BODY).expect("body");
        let left = layout.rect(SLOT_LEFT).expect("left");
        let middle = layout.rect(SLOT_MIDDLE).expect("middle");
        let right = layout.rect(SLOT_RIGHT).expect("right");

        assert_eq!(header.height, 3);
        assert_eq!(footer.height, 4);
        assert_eq!(body.height, 33);
        assert_eq!(left.height + header.height + footer.height, 40);
        assert_eq!(left.x, 0);
        assert_eq!(middle.x, left.width);
        assert_eq!(right.x, left.width + middle.width);
    }
}
