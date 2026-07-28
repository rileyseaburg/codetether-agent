//! Shared user-timing timeline storage.

use super::*;

thread_local! {
    static TIMELINE: RefCell<Timeline> = RefCell::new(Timeline::default());
}

#[derive(Default)]
pub(super) struct Timeline {
    pub(super) clock: f64,
    pub(super) entries: Vec<PerformanceEntry>,
}

pub(super) fn reset() {
    TIMELINE.with(|timeline| *timeline.borrow_mut() = Timeline::default());
}

pub(super) fn now() -> f64 {
    with_mut(tick)
}

pub(super) fn tick(timeline: &mut Timeline) -> f64 {
    let now = timeline.clock;
    timeline.clock += 1.0;
    now
}

pub(super) fn with<R>(f: impl FnOnce(&Timeline) -> R) -> R {
    TIMELINE.with(|timeline| f(&timeline.borrow()))
}

pub(super) fn with_mut<R>(f: impl FnOnce(&mut Timeline) -> R) -> R {
    TIMELINE.with(|timeline| f(&mut timeline.borrow_mut()))
}
