use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["key", "oldValue", "newValue", "storageArea"] {
        map.insert(name.into(), base::prop(init, name).unwrap_or(JsValue::Null));
    }
    map.insert(
        "url".into(),
        JsValue::String(base::string_prop(init, "url")),
    );
}
