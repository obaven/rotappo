use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use std::{cell::UnsafeCell, sync::atomic::{AtomicBool, Ordering}};

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
            if let Some(rect) = slot_rect(slot, &rows, &cols) {
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
    Layout::default().direction(direction).constraints(constraints).split(area)
}

fn slot_rect(slot: &GridSlot, rows: &[Rect], cols: &[Rect]) -> Option<Rect> {
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
    Some(Rect::new(left, top, right.saturating_sub(left), bottom.saturating_sub(top)))
}

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinGuard<'_, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }
        SpinGuard { lock: self }
    }
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
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
}
