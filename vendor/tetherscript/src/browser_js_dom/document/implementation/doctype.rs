use super::super::super::*;

pub(super) fn object() -> JsValue {
    let mut obj = HashMap::new();
    insert_string(&mut obj, "name", "html");
    obj.insert("nodeType".into(), JsValue::Number(10.0));
    insert_string(&mut obj, "nodeName", "html");
    insert_string(&mut obj, "publicId", "");
    insert_string(&mut obj, "systemId", "");
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn insert_string(obj: &mut HashMap<String, JsValue>, key: &str, value: &str) {
    obj.insert(key.into(), JsValue::String(value.into()));
}
