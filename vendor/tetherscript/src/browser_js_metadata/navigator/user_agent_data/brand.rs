use super::*;

pub(super) fn list() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(vec![
        item("TetherScript", "0.1"),
        item("BrowserCompat", "0"),
    ])))
}

fn item(name: &str, version: &str) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("brand".into(), JsValue::String(name.into())),
        ("version".into(), JsValue::String(version.into())),
    ]))))
}
