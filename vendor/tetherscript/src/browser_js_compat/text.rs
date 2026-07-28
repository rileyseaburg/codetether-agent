use super::*;

#[path = "text/decoder.rs"]
mod decoder;
#[path = "text/destination.rs"]
mod destination;
#[path = "text/encoder.rs"]
mod encoder;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "TextEncoder".into(),
        native("TextEncoder", Some(0), move |_| Ok(encoder::object())),
    );
    window.insert(
        "TextDecoder".into(),
        native("TextDecoder", None, move |args| Ok(decoder::object(args))),
    );
}
