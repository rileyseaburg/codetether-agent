use super::*;

pub(super) fn message(data: JsValue, origin: &str, source: JsValue) -> JsValue {
    let map = HashMap::from([
        ("type".into(), JsValue::String("message".into())),
        ("data".into(), data),
        ("origin".into(), JsValue::String(origin.into())),
        ("source".into(), source),
        ("isTrusted".into(), JsValue::Bool(false)),
    ]);
    JsValue::Object(Rc::new(RefCell::new(map)))
}
