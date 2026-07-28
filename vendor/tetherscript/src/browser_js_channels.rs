//! Deterministic channel and worker host shims for browser JavaScript.

use super::*;

#[path = "browser_js_channels/mod.rs"]
mod inner;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    inner::install(window, timers);
}

pub(super) fn reset_all() {
    inner::reset_all();
}
