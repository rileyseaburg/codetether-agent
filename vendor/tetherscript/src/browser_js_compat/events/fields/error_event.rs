use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["message", "filename"] {
        map.insert(name.into(), JsValue::String(base::string_prop(init, name)));
    }
    for name in ["lineno", "colno"] {
        map.insert(
            name.into(),
            JsValue::Number(base::num_prop(init, name, 0.0)),
        );
    }
    map.insert(
        "error".into(),
        base::prop(init, "error").unwrap_or(JsValue::Null),
    );
}
