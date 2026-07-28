use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in [
        "screenX", "screenY", "clientX", "clientY", "pageX", "pageY", "offsetX", "offsetY",
        "button", "buttons", "detail",
    ] {
        map.insert(
            name.into(),
            JsValue::Number(base::num_prop(init, name, 0.0)),
        );
    }
    for name in ["ctrlKey", "shiftKey", "altKey", "metaKey"] {
        map.insert(name.into(), JsValue::Bool(base::bool_prop(init, name)));
    }
}
