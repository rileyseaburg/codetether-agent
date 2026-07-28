use super::*;

pub(super) fn drag(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "dataTransfer".into(),
        base::prop(init, "dataTransfer").unwrap_or(JsValue::Null),
    );
}

pub(super) fn composition(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "data".into(),
        JsValue::String(base::string_prop(init, "data")),
    );
}

pub(super) fn touch(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["touches", "targetTouches", "changedTouches"] {
        map.insert(
            name.into(),
            base::prop(init, name).unwrap_or_else(empty_array),
        );
    }
    for name in ["altKey", "ctrlKey", "metaKey", "shiftKey"] {
        map.insert(name.into(), JsValue::Bool(base::bool_prop(init, name)));
    }
}

fn empty_array() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(Vec::new())))
}
