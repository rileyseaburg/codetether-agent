use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "clipboardData".into(),
        base::prop(init, "clipboardData").unwrap_or(JsValue::Null),
    );
}
