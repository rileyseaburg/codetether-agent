use super::*;

pub(super) fn install(
    object: &JsObject,
    id: u64,
    name: String,
    origin: String,
    timers: Rc<RefCell<TimerQueue>>,
) {
    let close_object = object.clone();
    object.borrow_mut().insert(
        "close".into(),
        native("BroadcastChannel.close", Some(0), move |_| {
            mark_closed(&close_object);
            Ok(JsValue::Undefined)
        }),
    );
    object.borrow_mut().insert(
        "postMessage".into(),
        native("BroadcastChannel.postMessage", Some(1), move |args| {
            post(id, &name, &origin, timers.clone(), args);
            Ok(JsValue::Undefined)
        }),
    );
}

fn post(id: u64, name: &str, origin: &str, timers: Rc<RefCell<TimerQueue>>, args: &[JsValue]) {
    let data = args.first().cloned().unwrap_or(JsValue::Undefined);
    for target in broadcast_state::targets(id, name, origin) {
        port::queue(
            timers.clone(),
            target,
            data.clone(),
            origin.into(),
            JsValue::Null,
        );
    }
}
