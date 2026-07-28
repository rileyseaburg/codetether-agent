use super::*;

pub(super) use delivery::queue;

pub(super) fn pair(timers: Rc<RefCell<TimerQueue>>) -> (JsValue, JsValue) {
    let left = bare();
    let right = bare();
    install_methods(&left, right.clone(), timers.clone());
    install_methods(&right, left.clone(), timers);
    (JsValue::Object(left), JsValue::Object(right))
}

fn bare() -> JsObject {
    Rc::new(RefCell::new(HashMap::from([
        ("onmessage".into(), JsValue::Null),
        ("__closed".into(), JsValue::Bool(false)),
    ])))
}

fn install_methods(object: &JsObject, peer: JsObject, timers: Rc<RefCell<TimerQueue>>) {
    listeners::install(object, "MessagePort");
    let post_timers = timers;
    let post_peer = peer;
    object.borrow_mut().insert(
        "postMessage".into(),
        native("MessagePort.postMessage", Some(1), move |args| {
            let data = args.first().cloned().unwrap_or(JsValue::Undefined);
            queue(
                post_timers.clone(),
                post_peer.clone(),
                data,
                String::new(),
                JsValue::Null,
            );
            Ok(JsValue::Undefined)
        }),
    );
    let close_object = object.clone();
    object.borrow_mut().insert(
        "start".into(),
        native("MessagePort.start", Some(0), |_| Ok(JsValue::Undefined)),
    );
    object.borrow_mut().insert(
        "close".into(),
        native("MessagePort.close", Some(0), move |_| {
            mark_closed(&close_object);
            Ok(JsValue::Undefined)
        }),
    );
}
