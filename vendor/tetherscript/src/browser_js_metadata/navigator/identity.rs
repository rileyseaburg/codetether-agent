use super::*;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert("webdriver".into(), JsValue::Bool(false));
    navigator.insert("maxTouchPoints".into(), JsValue::Number(0.0));
    set_str(navigator, "vendor", "TetherScript");
    set_str(navigator, "product", "Gecko");
}
