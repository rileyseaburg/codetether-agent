use super::*;

pub(super) fn insert(object: &mut HashMap<String, JsValue>, hint: &str) {
    match hint {
        "architecture" => set_str(object, hint, "x86"),
        "bitness" => set_str(object, hint, "64"),
        "model" => set_str(object, hint, ""),
        "platformVersion" => set_str(object, hint, "0"),
        "uaFullVersion" => set_str(object, hint, "0.1"),
        "fullVersionList" => {
            object.insert(hint.into(), brand::list());
        }
        "wow64" => {
            object.insert(hint.into(), JsValue::Bool(false));
        }
        _ => {}
    }
}
