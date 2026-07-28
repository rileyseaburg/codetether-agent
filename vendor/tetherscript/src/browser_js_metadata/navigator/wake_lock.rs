use super::*;
use std::{cell::RefCell, rc::Rc};

#[path = "wake_lock/sentinel.rs"]
mod sentinel;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert(
        "wakeLock".into(),
        JsValue::Object(Rc::new(RefCell::new(HashMap::from([(
            "request".into(),
            native("navigator.wakeLock.request", None, request),
        )])))),
    );
}

fn request(args: &[JsValue]) -> Result<JsValue, String> {
    let kind = args
        .first()
        .map_or_else(|| "undefined".into(), JsValue::display);
    if kind != "screen" {
        return Ok(reject(kind));
    }
    Ok(thenable::resolved(sentinel::new(kind)))
}

fn reject(kind: String) -> JsValue {
    let reason = format!("navigator.wakeLock.request: unsupported type '{kind}'");
    rejection::thenable(JsValue::String(reason))
}
