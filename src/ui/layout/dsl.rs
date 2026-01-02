#[macro_export]
macro_rules! grid_spec {
    (rows: [$($row:expr),* $(,)?], cols: [$($col:expr),* $(,)?], slots: [$($slot:expr),* $(,)?]) => {{
        $crate::ui::layout::GridSpec::new(vec![$($row),*], vec![$($col),*])
            .with_slots(vec![$($slot),*])
    }};
}

#[macro_export]
macro_rules! grid_slot {
    ($id:expr, $row:expr, $col:expr $(, $($rest:tt)*)? ) => {{
        let slot = $crate::ui::layout::GridSlot::new($id, $row, $col);
        $crate::grid_slot_opts!(slot $(, $($rest)*)?)
    }};
}

#[macro_export]
macro_rules! grid_slot_opts {
    ($slot:expr) => { $slot };
    ($slot:expr, span: ($r:expr, $c:expr) $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.span($r, $c) $(, $($rest)*)?)
    };
    ($slot:expr, hidden $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.hidden() $(, $($rest)*)?)
    };
    ($slot:expr, movable: $val:expr $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.movable($val) $(, $($rest)*)?)
    };
    ($slot:expr, min: ($w:expr, $h:expr) $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.with_min_size($w, $h) $(, $($rest)*)?)
    };
    ($slot:expr, max: ($w:expr, $h:expr) $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.with_max_size($w, $h) $(, $($rest)*)?)
    };
    ($slot:expr, offset: ($x:expr, $y:expr) $(, $($rest:tt)*)? ) => {
        $crate::grid_slot_opts!($slot.offset($x, $y) $(, $($rest)*)?)
    };
}

#[cfg(test)]
mod tests {
    use crate::ui::layout::TrackSize;

    #[test]
    fn builds_spec_with_macros() {
        let spec = crate::grid_spec!(
            rows: [TrackSize::Fixed(2), TrackSize::Fixed(2)],
            cols: [TrackSize::Fixed(3)],
            slots: [
                crate::grid_slot!("header", 0, 0, span: (1, 1)),
                crate::grid_slot!("body", 1, 0, movable: true, min: (2, 2)),
            ]
        );

        assert_eq!(spec.rows.len(), 2);
        assert_eq!(spec.cols.len(), 1);
        assert_eq!(spec.slots.len(), 2);
        assert_eq!(spec.slots[0].id.as_str(), "header");
        assert!(spec.slots[1].movable);
        assert_eq!(spec.slots[1].min_width, Some(2));
    }
}
