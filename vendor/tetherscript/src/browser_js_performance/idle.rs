//! Deterministic idle callback scheduling.

use super::*;

#[path = "idle_drain.rs"]
mod idle_drain;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    let request_queue = timers.clone();
    window.insert(
        "requestIdleCallback".into(),
        native("requestIdleCallback", None, move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let deadline = idle_deadline::object(args.get(1));
            let mut queue = request_queue.borrow_mut();
            queue.next_id = queue.next_id.saturating_add(1).max(1);
            let id = queue.next_id;
            queue.idle_callbacks.push_back(ScheduledCallback {
                id,
                callback,
                args: vec![deadline],
                this_value: JsValue::Undefined,
            });
            Ok(JsValue::Number(id as f64))
        }),
    );
    window.insert(
        "cancelIdleCallback".into(),
        native("cancelIdleCallback", Some(1), move |args| {
            let id = args.first().map(timer_id).unwrap_or(0);
            timers
                .borrow_mut()
                .idle_callbacks
                .retain(|task| task.id != id);
            Ok(JsValue::Undefined)
        }),
    );
}

pub(super) fn drain(
    timers: Rc<RefCell<TimerQueue>>,
    window: JsValue,
    drained: &mut usize,
) -> Result<bool, String> {
    idle_drain::drain(timers, window, drained)
}
