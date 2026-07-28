use super::*;

pub(super) fn new(kind: String) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("type".into(), JsValue::String(kind)),
        ("released".into(), JsValue::Bool(false)),
        ("onrelease".into(), JsValue::Null),
    ])));
    let release = release(object.clone());
    let mut data = object.borrow_mut();
    data.insert("release".into(), release);
    events(&mut data);
    drop(data);
    JsValue::Object(object)
}

fn release(object: Rc<RefCell<HashMap<String, JsValue>>>) -> JsValue {
    native("WakeLockSentinel.release", Some(0), move |_| {
        object
            .borrow_mut()
            .insert("released".into(), JsValue::Bool(true));
        Ok(thenable::resolved(JsValue::Undefined))
    })
}

fn events(data: &mut HashMap<String, JsValue>) {
    for method in ["addEventListener", "removeEventListener"] {
        data.insert(method.into(), noop(method));
    }
    data.insert(
        "dispatchEvent".into(),
        native("WakeLockSentinel.dispatchEvent", Some(1), |_| {
            Ok(JsValue::Bool(true))
        }),
    );
}

fn noop(method: &str) -> JsValue {
    let name = format!("WakeLockSentinel.{method}");
    native(&name, None, |_| Ok(JsValue::Undefined))
}
