use super::*;

pub(super) fn input(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "data".into(),
        base::prop(init, "data").unwrap_or(JsValue::Null),
    );
    map.insert(
        "inputType".into(),
        JsValue::String(base::string_prop(init, "inputType")),
    );
    map.insert(
        "isComposing".into(),
        JsValue::Bool(base::bool_prop(init, "isComposing")),
    );
}
