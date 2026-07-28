use super::*;

pub(super) fn cookie(name: String, value: String) -> JsValue {
    let mut object = HashMap::new();
    object.insert("name".into(), JsValue::String(name));
    object.insert("value".into(), JsValue::String(value));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

pub(super) fn array(pairs: Vec<(String, String)>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        pairs
            .into_iter()
            .map(|(name, value)| cookie(name, value))
            .collect(),
    )))
}
