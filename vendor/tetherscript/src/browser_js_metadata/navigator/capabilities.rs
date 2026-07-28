use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert("hardwareConcurrency".into(), JsValue::Number(4.0));
    navigator.insert("deviceMemory".into(), JsValue::Number(8.0));
    navigator.insert("onLine".into(), JsValue::Bool(true));
    navigator.insert("cookieEnabled".into(), JsValue::Bool(true));
    navigator.insert("pdfViewerEnabled".into(), JsValue::Bool(false));
    navigator.insert("doNotTrack".into(), JsValue::Null);
    navigator.insert("plugins".into(), empty_array());
    navigator.insert("mimeTypes".into(), empty_array());
    navigator.insert(
        "javaEnabled".into(),
        native("navigator.javaEnabled", Some(0), |_| {
            Ok(JsValue::Bool(false))
        }),
    );
    set_str(navigator, "language", "en-US");
    set_str(navigator, "platform", "TetherScript");
    set_str(navigator, "userAgent", "TetherScript/0.1 BrowserCompat");
    set_str(navigator, "appCodeName", "Mozilla");
    set_str(navigator, "appName", "Netscape");
    set_str(navigator, "appVersion", "TetherScript/0.1 BrowserCompat");
    navigator.insert(
        "languages".into(),
        JsValue::Array(Rc::new(RefCell::new(vec![
            JsValue::String("en-US".into()),
            JsValue::String("en".into()),
        ]))),
    );
    navigator.insert("userActivation".into(), activation());
}

fn empty_array() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(Vec::new())))
}

fn activation() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("isActive".into(), JsValue::Bool(false)),
        ("hasBeenActive".into(), JsValue::Bool(false)),
    ]))))
}
