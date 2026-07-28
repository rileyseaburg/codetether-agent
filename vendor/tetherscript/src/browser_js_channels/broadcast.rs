use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    let origin = location_origin(window);
    window.insert(
        "BroadcastChannel".into(),
        native("BroadcastChannel", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(channel(&name, &origin, timers.clone()))
        }),
    );
}

pub(super) fn reset_all() {
    broadcast_state::reset_all();
}

fn channel(name: &str, origin: &str, timers: Rc<RefCell<TimerQueue>>) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("name".into(), JsValue::String(name.into())),
        ("origin".into(), JsValue::String(origin.into())),
        ("onmessage".into(), JsValue::Null),
        ("__closed".into(), JsValue::Bool(false)),
    ])));
    listeners::install(&object, "BroadcastChannel");
    let id = broadcast_state::register(name, origin, object.clone());
    broadcast_methods::install(&object, id, name.into(), origin.into(), timers);
    JsValue::Object(object)
}
