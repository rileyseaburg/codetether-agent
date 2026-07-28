use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "data".into(),
        base::prop(init, "data").unwrap_or(JsValue::Null),
    );
    for name in ["origin", "lastEventId"] {
        map.insert(name.into(), JsValue::String(base::string_prop(init, name)));
    }
    map.insert(
        "source".into(),
        base::prop(init, "source").unwrap_or(JsValue::Null),
    );
    map.insert(
        "ports".into(),
        base::prop(init, "ports").unwrap_or_else(empty_array),
    );
}

fn empty_array() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(Vec::new())))
}
