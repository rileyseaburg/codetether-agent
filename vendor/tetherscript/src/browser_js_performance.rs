//! Deterministic Performance, idle, and user-timing host shims.

use super::*;

#[path = "browser_js_performance/clear.rs"]
mod clear;
#[path = "browser_js_performance/entry.rs"]
mod entry;
#[path = "browser_js_performance/entry_json.rs"]
mod entry_json;
#[path = "browser_js_performance/idle.rs"]
mod idle;
#[path = "browser_js_performance/idle_deadline.rs"]
mod idle_deadline;
#[path = "browser_js_performance/list.rs"]
mod list;
#[path = "browser_js_performance/marks.rs"]
mod marks;
#[path = "browser_js_performance/observer.rs"]
mod observer;
#[path = "browser_js_performance/performance.rs"]
mod performance;
#[path = "browser_js_performance/read.rs"]
mod read;
#[path = "browser_js_performance/state.rs"]
mod state;

use entry::PerformanceEntry;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    performance::install(window, timers.clone());
    idle::install(window, timers.clone());
    observer::install(window, timers);
}

pub(super) fn reset() {
    state::reset();
    observer::reset();
}

pub(super) fn drain_idle_callbacks(
    timers: Rc<RefCell<TimerQueue>>,
    window: JsValue,
    drained: &mut usize,
) -> Result<bool, String> {
    idle::drain(timers, window, drained)
}
