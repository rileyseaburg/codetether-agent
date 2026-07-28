use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    window.insert(
        "MessageChannel".into(),
        native("MessageChannel", Some(0), move |_| {
            let (port1, port2) = port::pair(timers.clone());
            let map = HashMap::from([("port1".into(), port1), ("port2".into(), port2)]);
            Ok(JsValue::Object(Rc::new(RefCell::new(map))))
        }),
    );
}
