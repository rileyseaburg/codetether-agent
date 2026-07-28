use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "wasClean".into(),
        JsValue::Bool(base::bool_prop(init, "wasClean")),
    );
    map.insert(
        "code".into(),
        JsValue::Number(base::num_prop(init, "code", 0.0)),
    );
    map.insert(
        "reason".into(),
        JsValue::String(base::string_prop(init, "reason")),
    );
}
