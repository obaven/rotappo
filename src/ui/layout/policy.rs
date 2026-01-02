use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ratatui::layout::Rect;

use crate::ui::layout::{GridSlot, GridSpec, SlotId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelPriority {
    Critical,
    High,
    Normal,
    Low,
}

impl PanelPriority {
    pub fn rank(self) -> u8 {
        match self {
            PanelPriority::Critical => 4,
            PanelPriority::High => 3,
            PanelPriority::Normal => 2,
            PanelPriority::Low => 1,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SlotOverride {
    pub visible: Option<bool>,
    pub collapsed: Option<bool>,
    pub row: Option<usize>,
    pub col: Option<usize>,
    pub row_span: Option<usize>,
    pub col_span: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SlotPolicy {
    pub priority: PanelPriority,
    pub movable: bool,
    pub min_width: Option<u16>,
    pub min_height: Option<u16>,
    pub max_width: Option<u16>,
    pub max_height: Option<u16>,
}

impl SlotPolicy {
    pub fn new(priority: PanelPriority) -> Self {
        Self {
            priority,
            movable: false,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
        }
    }

    pub fn movable(mut self, value: bool) -> Self {
        self.movable = value;
        self
    }

    pub fn min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = Some(width);
        self.min_height = Some(height);
        self
    }

    pub fn max_size(mut self, width: u16, height: u16) -> Self {
        self.max_width = Some(width);
        self.max_height = Some(height);
        self
    }
}

#[derive(Clone)]
pub struct LayoutPolicy {
    state: Arc<RwLock<PolicyState>>,
}

impl LayoutPolicy {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(PolicyState::default())),
        }
    }

    pub fn set_policy(&self, slot: impl Into<SlotId>, policy: SlotPolicy) {
        if let Ok(mut guard) = self.state.write() {
            guard.policies.insert(slot.into(), policy);
        }
    }

    pub fn set_override(&self, slot: impl Into<SlotId>, override_data: SlotOverride) {
        if let Ok(mut guard) = self.state.write() {
            guard.overrides.insert(slot.into(), override_data);
        }
    }

    pub fn clear_override(&self, slot: &SlotId) {
        if let Ok(mut guard) = self.state.write() {
            guard.overrides.remove(slot);
        }
    }

    pub fn set_visibility(&self, slot: impl Into<SlotId>, visible: bool) {
        let slot = slot.into();
        if let Ok(mut guard) = self.state.write() {
            let entry = guard.overrides.entry(slot).or_default();
            entry.visible = Some(visible);
        }
    }

    pub fn visibility_for(&self, slot: &str) -> Option<bool> {
        let guard = self.state.read().ok()?;
        guard.overrides.get(slot).and_then(|entry| entry.visible)
    }

    pub fn set_collapsed(&self, slot: impl Into<SlotId>, collapsed: bool) {
        let slot = slot.into();
        if let Ok(mut guard) = self.state.write() {
            let entry = guard.overrides.entry(slot).or_default();
            entry.collapsed = Some(collapsed);
        }
    }

    pub fn collapsed_for(&self, slot: &str) -> Option<bool> {
        let guard = self.state.read().ok()?;
        guard.overrides.get(slot).and_then(|entry| entry.collapsed)
    }

    pub fn set_position(&self, slot: impl Into<SlotId>, row: usize, col: usize) {
        let slot = slot.into();
        if let Ok(mut guard) = self.state.write() {
            let entry = guard.overrides.entry(slot).or_default();
            entry.row = Some(row);
            entry.col = Some(col);
        }
    }

    pub fn set_span(&self, slot: impl Into<SlotId>, row_span: usize, col_span: usize) {
        let slot = slot.into();
        if let Ok(mut guard) = self.state.write() {
            let entry = guard.overrides.entry(slot).or_default();
            entry.row_span = Some(row_span.max(1));
            entry.col_span = Some(col_span.max(1));
        }
    }

    pub fn apply(&self, spec: &GridSpec, _area: Rect) -> GridSpec {
        let guard = match self.state.read() {
            Ok(guard) => guard,
            Err(_) => return spec.clone(),
        };
        let mut next = spec.clone();
        for slot in next.slots.iter_mut() {
            if let Some(policy) = guard.policies.get(&slot.id) {
                slot.movable = policy.movable;
                slot.min_width = policy.min_width;
                slot.min_height = policy.min_height;
                slot.max_width = policy.max_width;
                slot.max_height = policy.max_height;
            }
            if let Some(override_data) = guard.overrides.get(&slot.id) {
                apply_override(slot, override_data);
            }
        }
        next
    }
}

#[derive(Default)]
struct PolicyState {
    policies: HashMap<SlotId, SlotPolicy>,
    overrides: HashMap<SlotId, SlotOverride>,
}

fn apply_override(slot: &mut GridSlot, override_data: &SlotOverride) {
    if let Some(visible) = override_data.visible {
        slot.visible = visible;
    }
    if let Some(row) = override_data.row {
        slot.row = row;
    }
    if let Some(col) = override_data.col {
        slot.col = col;
    }
    if let Some(row_span) = override_data.row_span {
        slot.row_span = row_span.max(1);
    }
    if let Some(col_span) = override_data.col_span {
        slot.col_span = col_span.max(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::layout::{GridSlot, GridSpec, TrackSize};

    #[test]
    fn applies_visibility_override() {
        let policy = LayoutPolicy::new();
        policy.set_visibility("slot", false);

        let spec = GridSpec::new(vec![TrackSize::Fixed(2)], vec![TrackSize::Fixed(2)])
            .with_slots(vec![GridSlot::new("slot", 0, 0)]);
        let updated = policy.apply(&spec, Rect::new(0, 0, 4, 4));
        assert_eq!(updated.slots.len(), 1);
        assert!(!updated.slots[0].visible);
    }

    #[test]
    fn applies_position_and_span_override() {
        let policy = LayoutPolicy::new();
        policy.set_position("slot", 1, 2);
        policy.set_span("slot", 2, 3);

        let spec = GridSpec::new(
            vec![TrackSize::Fixed(2), TrackSize::Fixed(2), TrackSize::Fixed(2)],
            vec![TrackSize::Fixed(2), TrackSize::Fixed(2), TrackSize::Fixed(2)],
        )
        .with_slots(vec![GridSlot::new("slot", 0, 0)]);
        let updated = policy.apply(&spec, Rect::new(0, 0, 6, 6));
        assert_eq!(updated.slots[0].row, 1);
        assert_eq!(updated.slots[0].col, 2);
        assert_eq!(updated.slots[0].row_span, 2);
        assert_eq!(updated.slots[0].col_span, 3);
    }

    #[test]
    fn applies_policy_attributes() {
        let policy = LayoutPolicy::new();
        policy.set_policy(
            "slot",
            SlotPolicy::new(PanelPriority::High)
                .movable(true)
                .min_size(4, 3)
                .max_size(8, 6),
        );

        let spec = GridSpec::new(vec![TrackSize::Fixed(10)], vec![TrackSize::Fixed(10)])
            .with_slots(vec![GridSlot::new("slot", 0, 0)]);
        let updated = policy.apply(&spec, Rect::new(0, 0, 10, 10));
        let slot = &updated.slots[0];
        assert!(slot.movable);
        assert_eq!(slot.min_width, Some(4));
        assert_eq!(slot.min_height, Some(3));
        assert_eq!(slot.max_width, Some(8));
        assert_eq!(slot.max_height, Some(6));
    }
}
