use super::*;

pub(super) fn insert(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["promise", "reason"] {
        map.insert(name.into(), base::prop(init, name).unwrap_or(JsValue::Null));
    }
}
