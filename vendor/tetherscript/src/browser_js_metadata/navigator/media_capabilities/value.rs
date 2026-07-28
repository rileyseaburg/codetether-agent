use crate::js::JsValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) fn from_config(config: Option<&JsValue>) -> JsValue {
    let mut output = HashMap::from([
        ("supported".into(), JsValue::Bool(config.is_some())),
        ("smooth".into(), JsValue::Bool(true)),
        ("powerEfficient".into(), JsValue::Bool(true)),
    ]);
    if let Some(JsValue::Object(object)) = config {
        let object = object.borrow();
        copy_field(&object, &mut output, "type");
        if let Some(media) = media_object(&object) {
            let media = media.borrow();
            copy_field(&media, &mut output, "contentType");
        }
    }
    JsValue::Object(Rc::new(RefCell::new(output)))
}

fn media_object(
    config: &HashMap<String, JsValue>,
) -> Option<Rc<RefCell<HashMap<String, JsValue>>>> {
    match config.get("video").or_else(|| config.get("audio")) {
        Some(JsValue::Object(object)) => Some(object.clone()),
        _ => None,
    }
}

fn copy_field(
    source: &HashMap<String, JsValue>,
    output: &mut HashMap<String, JsValue>,
    field: &str,
) {
    if let Some(value) = source.get(field) {
        output.insert(field.into(), value.clone());
    }
}
