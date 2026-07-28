use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "matches".into(),
        JsValue::Bool(base::bool_prop(init, "matches")),
    );
    map.insert(
        "media".into(),
        JsValue::String(base::string_prop(init, "media")),
    );
}
