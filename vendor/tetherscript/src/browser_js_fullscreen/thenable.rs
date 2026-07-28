use super::*;

pub(super) fn resolved(value: JsValue) -> JsValue {
    let mut promise = HashMap::new();
    promise.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    promise.insert("__promise_value".into(), value.clone());
    install_then_catch_simple(&mut promise, value);
    JsValue::Object(Rc::new(RefCell::new(promise)))
}
