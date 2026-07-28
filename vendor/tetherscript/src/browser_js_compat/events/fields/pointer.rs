use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "pointerId".into(),
        JsValue::Number(base::num_prop(init, "pointerId", 0.0)),
    );
    map.insert(
        "pointerType".into(),
        JsValue::String(base::string_prop(init, "pointerType")),
    );
    map.insert(
        "isPrimary".into(),
        JsValue::Bool(base::bool_prop(init, "isPrimary")),
    );
    map.insert(
        "pressure".into(),
        JsValue::Number(base::num_prop(init, "pressure", 0.0)),
    );
}
