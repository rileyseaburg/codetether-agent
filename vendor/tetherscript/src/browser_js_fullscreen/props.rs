use super::*;

pub(super) fn write(
    obj: &mut HashMap<String, JsValue>,
    fullscreen: Option<&target::Target>,
    pointer: Option<&target::Target>,
) {
    obj.insert("fullscreenEnabled".into(), JsValue::Bool(true));
    obj.insert("fullscreenElement".into(), value(fullscreen));
    obj.insert("pointerLockElement".into(), value(pointer));
}

fn value(target: Option<&target::Target>) -> JsValue {
    target.map(target::Target::value).unwrap_or(JsValue::Null)
}
