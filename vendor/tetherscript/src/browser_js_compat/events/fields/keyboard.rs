use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["key", "code"] {
        map.insert(name.into(), JsValue::String(base::string_prop(init, name)));
    }
    map.insert(
        "location".into(),
        JsValue::Number(base::num_prop(init, "location", 0.0)),
    );
    for name in ["repeat", "ctrlKey", "shiftKey", "altKey", "metaKey"] {
        map.insert(name.into(), JsValue::Bool(base::bool_prop(init, name)));
    }
}
