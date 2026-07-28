use super::*;

pub(super) fn object(file: &AgentFile) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("name".into(), JsValue::String(file.name.clone()));
    obj.insert("type".into(), JsValue::String(file.mime_type.clone()));
    obj.insert("size".into(), JsValue::Number(file.size));
    obj.insert("lastModified".into(), JsValue::Number(file.last_modified));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
