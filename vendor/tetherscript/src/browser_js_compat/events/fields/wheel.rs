use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["deltaX", "deltaY", "deltaZ", "deltaMode"] {
        map.insert(
            name.into(),
            JsValue::Number(base::num_prop(init, name, 0.0)),
        );
    }
}
