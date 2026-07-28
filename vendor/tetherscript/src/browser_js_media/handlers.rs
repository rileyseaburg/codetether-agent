use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    for prop in [
        "onplay",
        "onplaying",
        "onpause",
        "onloadstart",
        "onloadedmetadata",
        "onloadeddata",
        "oncanplay",
        "ontimeupdate",
        "onended",
        "onvolumechange",
        "onratechange",
        "onerror",
    ] {
        obj.insert(prop.into(), JsValue::Null);
        let h = handle.clone();
        obj.insert(
            format!("__set:{prop}"),
            native(&format!("set_media_{prop}"), Some(1), move |args| {
                h.set_handler(prop, args.first().cloned().unwrap_or(JsValue::Undefined));
                Ok(JsValue::Undefined)
            }),
        );
    }
}
