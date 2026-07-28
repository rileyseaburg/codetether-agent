use super::*;

pub(super) fn create() -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("EMPTY".into(), JsValue::Number(0.0)),
        ("LOADING".into(), JsValue::Number(1.0)),
        ("DONE".into(), JsValue::Number(2.0)),
        ("readyState".into(), JsValue::Number(0.0)),
        ("result".into(), JsValue::Null),
        ("error".into(), JsValue::Null),
    ])));
    let listeners = Rc::new(RefCell::new(Vec::new()));
    events::install_methods(&object, listeners.clone());
    read::install(&object, listeners);
    JsValue::Object(object)
}
