use super::*;

pub(super) fn queue(
    timers: Rc<RefCell<TimerQueue>>,
    target: JsObject,
    data: JsValue,
    origin: String,
    source: JsValue,
) {
    let callback = native("MessagePort.dispatch", Some(0), move |_| {
        if !closed(&target) {
            let event = event::message(data.clone(), &origin, source.clone());
            listeners::dispatch(&target, "message", event)?;
        }
        Ok(JsValue::Undefined)
    });
    timers.borrow_mut().microtasks.push_back(ScheduledCallback {
        id: 0,
        callback,
        args: Vec::new(),
        this_value: JsValue::Undefined,
    });
}
