use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    for (name, value) in [
        ("outerWidth", constants::DEFAULT_VIEWPORT_WIDTH as f64),
        ("outerHeight", constants::DEFAULT_VIEWPORT_HEIGHT as f64),
        ("screenX", 0.0),
        ("screenY", 0.0),
        ("screenLeft", 0.0),
        ("screenTop", 0.0),
    ] {
        window.insert(name.into(), JsValue::Number(value));
    }
}
