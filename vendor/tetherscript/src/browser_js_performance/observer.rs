//! Minimal PerformanceObserver host.

use super::*;

#[path = "observer/object.rs"]
mod object;
#[path = "observer/store.rs"]
mod store;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    window.insert(
        "PerformanceObserver".into(),
        native("PerformanceObserver", Some(1), move |args| {
            Ok(object::create(
                args.first().cloned().unwrap_or(JsValue::Undefined),
                timers.clone(),
            ))
        }),
    );
}

pub(super) fn reset() {
    store::reset();
}

pub(super) fn notify(entry: PerformanceEntry, timers: Rc<RefCell<TimerQueue>>) {
    for delivery in store::record(entry) {
        queue_delivery(timers.clone(), delivery);
    }
}

fn queue_delivery(timers: Rc<RefCell<TimerQueue>>, delivery: store::Delivery) {
    let callback = native("PerformanceObserver.dispatch", Some(0), move |_| {
        let records = store::take(delivery.id);
        if records.is_empty() {
            return Ok(JsValue::Undefined);
        }
        let list = list::object(records);
        js::call_function_with_this(
            delivery.callback.clone(),
            JsValue::Undefined,
            &[list, delivery.object.clone()],
        )?;
        Ok(JsValue::Undefined)
    });
    timers.borrow_mut().microtasks.push_back(ScheduledCallback {
        id: 0,
        callback,
        args: Vec::new(),
        this_value: JsValue::Undefined,
    });
}
