use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlotId(String);

impl SlotId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SlotId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Borrow<str> for SlotId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackSize {
    Fixed(u16),
    Percent(u16),
    Min(u16),
    Max(u16),
    Fill(u16),
}

impl TrackSize {
    fn to_constraint(self) -> Constraint {
        match self {
            TrackSize::Fixed(size) => Constraint::Length(size),
            TrackSize::Percent(value) => Constraint::Percentage(value),
            TrackSize::Min(size) => Constraint::Min(size),
            TrackSize::Max(size) => Constraint::Max(size),
            TrackSize::Fill(weight) => Constraint::Fill(weight),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GridSlot {
    pub id: SlotId,
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
    pub visible: bool,
    pub movable: bool,
    pub min_width: Option<u16>,
    pub min_height: Option<u16>,
    pub max_width: Option<u16>,
    pub max_height: Option<u16>,
    pub offset_x: i16,
    pub offset_y: i16,
}

impl GridSlot {
    pub fn new(id: impl Into<SlotId>, row: usize, col: usize) -> Self {
        Self {
            id: id.into(),
            row,
            col,
            row_span: 1,
            col_span: 1,
            visible: true,
            movable: false,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn span(mut self, row_span: usize, col_span: usize) -> Self {
        self.row_span = row_span.max(1);
        self.col_span = col_span.max(1);
        self
    }

    pub fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    pub fn movable(mut self, value: bool) -> Self {
        self.movable = value;
        self
    }

    pub fn with_min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = Some(width);
        self.min_height = Some(height);
        self
    }

    pub fn with_max_size(mut self, width: u16, height: u16) -> Self {
        self.max_width = Some(width);
        self.max_height = Some(height);
        self
    }

    pub fn offset(mut self, x: i16, y: i16) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }
}

#[derive(Clone, Debug)]
pub struct GridSpec {
    pub rows: Vec<TrackSize>,
    pub cols: Vec<TrackSize>,
    pub slots: Vec<GridSlot>,
}

impl GridSpec {
    pub fn new(rows: Vec<TrackSize>, cols: Vec<TrackSize>) -> Self {
        Self {
            rows,
            cols,
            slots: Vec::new(),
        }
    }

    pub fn with_slots(mut self, slots: Vec<GridSlot>) -> Self {
        self.slots = slots;
        self
    }
}

#[derive(Clone, Debug)]
pub struct GridLayout {
    pub area: Rect,
    pub rects: HashMap<SlotId, Rect>,
    pub resolved_at: Instant,
}

impl GridLayout {
    pub fn rect(&self, id: &str) -> Option<Rect> {
        self.rects
            .iter()
            .find(|(key, _)| key.as_str() == id)
            .map(|(_, value)| *value)
    }
}

pub struct GridResolver;

impl GridResolver {
    pub fn resolve(area: Rect, spec: &GridSpec) -> GridLayout {
        let rows = resolve_tracks(area, Direction::Vertical, &spec.rows);
        let cols = resolve_tracks(area, Direction::Horizontal, &spec.cols);
        let mut rects = HashMap::new();

        for slot in &spec.slots {
            if !slot.visible {
                continue;
            }
            if let Some(rect) = slot_rect(area, slot, &rows, &cols) {
                rects.insert(slot.id.clone(), rect);
            }
        }

        GridLayout {
            area,
            rects,
            resolved_at: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct GridCache {
    inner: Arc<RwLock<GridCacheState>>,
}

impl GridCache {
    pub fn new(spec: GridSpec) -> Self {
        let layout = GridResolver::resolve(Rect::default(), &spec);
        Self {
            inner: Arc::new(RwLock::new(GridCacheState {
                spec,
                layout,
                last_area: Rect::default(),
            })),
        }
    }

    pub fn update_spec(&self, spec: GridSpec) {
        if let Ok(mut guard) = self.inner.write() {
            guard.spec = spec;
            guard.last_area = Rect::default();
        }
    }

    pub fn resolve(&self, area: Rect) -> GridLayout {
        if let Ok(mut guard) = self.inner.write() {
            if guard.last_area != area {
                guard.layout = GridResolver::resolve(area, &guard.spec);
                guard.last_area = area;
            }
            return guard.layout.clone();
        }
        GridLayout {
            area,
            rects: HashMap::new(),
            resolved_at: Instant::now(),
        }
    }

    pub fn snapshot(&self) -> GridLayout {
        if let Ok(guard) = self.inner.read() {
            return guard.layout.clone();
        }
        GridLayout {
            area: Rect::default(),
            rects: HashMap::new(),
            resolved_at: Instant::now(),
        }
    }

    pub async fn resolve_async(&self, area: Rect) -> GridLayout {
        let cache = self.clone();
        let task = tokio::task::spawn_blocking(move || cache.resolve(area));
        match task.await {
            Ok(layout) => layout,
            Err(_) => self.snapshot(),
        }
    }
}

struct GridCacheState {
    spec: GridSpec,
    layout: GridLayout,
    last_area: Rect,
}

fn resolve_tracks(area: Rect, direction: Direction, tracks: &[TrackSize]) -> Vec<Rect> {
    if tracks.is_empty() {
        return Vec::new();
    }
    let constraints: Vec<Constraint> = tracks.iter().map(|track| track.to_constraint()).collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
        .to_vec()
}

fn slot_rect(area: Rect, slot: &GridSlot, rows: &[Rect], cols: &[Rect]) -> Option<Rect> {
    if slot.row >= rows.len() || slot.col >= cols.len() {
        return None;
    }
    let row_end = slot.row.saturating_add(slot.row_span).min(rows.len());
    let col_end = slot.col.saturating_add(slot.col_span).min(cols.len());
    if row_end == slot.row || col_end == slot.col {
        return None;
    }
    let top = rows[slot.row].y;
    let left = cols[slot.col].x;
    let bottom = rows[row_end - 1].y.saturating_add(rows[row_end - 1].height);
    let right = cols[col_end - 1].x.saturating_add(cols[col_end - 1].width);
    let cell_width = right.saturating_sub(left);
    let cell_height = bottom.saturating_sub(top);
    let mut width = cell_width;
    let mut height = cell_height;
    if let Some(max_width) = slot.max_width {
        width = width.min(max_width);
    }
    if let Some(max_height) = slot.max_height {
        height = height.min(max_height);
    }
    if let Some(min_width) = slot.min_width {
        width = width.max(min_width).min(cell_width);
    }
    if let Some(min_height) = slot.min_height {
        height = height.max(min_height).min(cell_height);
    }
    let mut x = left;
    let mut y = top;
    if slot.movable && (slot.offset_x != 0 || slot.offset_y != 0) {
        let mut new_x = (x as i32).saturating_add(slot.offset_x as i32);
        let mut new_y = (y as i32).saturating_add(slot.offset_y as i32);
        let min_x = area.x as i32;
        let min_y = area.y as i32;
        let max_x = area
            .x
            .saturating_add(area.width)
            .saturating_sub(width) as i32;
        let max_y = area
            .y
            .saturating_add(area.height)
            .saturating_sub(height) as i32;
        if new_x < min_x {
            new_x = min_x;
        }
        if new_x > max_x {
            new_x = max_x;
        }
        if new_y < min_y {
            new_y = min_y;
        }
        if new_y > max_y {
            new_y = max_y;
        }
        x = new_x.max(0) as u16;
        y = new_y.max(0) as u16;
    }
    Some(Rect::new(x, y, width, height))
}

pub struct SpinLock<T> {
    inner: Mutex<T>,
}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn lock(&self) -> SpinGuard<'_, T> {
        loop {
            if let Ok(guard) = self.inner.try_lock() {
                return SpinGuard { guard };
            }
            std::hint::spin_loop();
        }
    }
}

pub struct SpinGuard<'a, T> {
    guard: std::sync::MutexGuard<'a, T>,
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_basic_grid() {
        let spec = GridSpec::new(
            vec![TrackSize::Percent(50), TrackSize::Percent(50)],
            vec![TrackSize::Percent(50), TrackSize::Percent(50)],
        )
        .with_slots(vec![
            GridSlot::new("a", 0, 0),
            GridSlot::new("b", 0, 1),
            GridSlot::new("c", 1, 0).span(1, 2),
        ]);

        let area = Rect::new(0, 0, 100, 40);
        let layout = GridResolver::resolve(area, &spec);
        let a = layout.rect("a").expect("slot a");
        let b = layout.rect("b").expect("slot b");
        let c = layout.rect("c").expect("slot c");

        assert_eq!(a.width, 50);
        assert_eq!(b.x, 50);
        assert_eq!(c.width, 100);
        assert_eq!(c.y, 20);
    }

    #[test]
    fn skips_hidden_slots() {
        let spec = GridSpec::new(
            vec![TrackSize::Fixed(2)],
            vec![TrackSize::Fixed(2)],
        )
        .with_slots(vec![GridSlot::new("hidden", 0, 0).hidden()]);
        let layout = GridResolver::resolve(Rect::new(0, 0, 4, 4), &spec);
        assert!(layout.rect("hidden").is_none());
    }

    #[test]
    fn applies_min_max_and_offsets() {
        let spec = GridSpec::new(vec![TrackSize::Fixed(10)], vec![TrackSize::Fixed(10)])
            .with_slots(vec![GridSlot::new("slot", 0, 0)
                .movable(true)
                .with_min_size(6, 6)
                .with_max_size(8, 8)
                .offset(3, 4)]);
        let area = Rect::new(0, 0, 10, 10);
        let layout = GridResolver::resolve(area, &spec);
        let rect = layout.rect("slot").expect("slot rect");
        assert_eq!(rect.width, 8);
        assert_eq!(rect.height, 8);
        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 2);
    }

    #[test]
    fn clamps_offsets_to_area() {
        let spec = GridSpec::new(vec![TrackSize::Fixed(6)], vec![TrackSize::Fixed(6)])
            .with_slots(vec![GridSlot::new("slot", 0, 0)
                .movable(true)
                .with_max_size(4, 4)
                .offset(10, 10)]);
        let area = Rect::new(0, 0, 6, 6);
        let layout = GridResolver::resolve(area, &spec);
        let rect = layout.rect("slot").expect("slot rect");
        assert_eq!(rect.width, 4);
        assert_eq!(rect.height, 4);
        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 2);
    }
}
