use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[path = "media_capabilities/value.rs"]
mod value;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    let methods = HashMap::from([
        (
            "decodingInfo".into(),
            native("navigator.mediaCapabilities.decodingInfo", Some(1), info),
        ),
        (
            "encodingInfo".into(),
            native("navigator.mediaCapabilities.encodingInfo", Some(1), info),
        ),
    ]);
    navigator.insert(
        "mediaCapabilities".into(),
        JsValue::Object(Rc::new(RefCell::new(methods))),
    );
}

fn info(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(thenable::resolved(value::from_config(args.first())))
}
