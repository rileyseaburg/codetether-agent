use super::*;

#[path = "blob/access.rs"]
pub(super) mod access;
#[path = "blob/methods.rs"]
mod methods;
#[path = "blob/object.rs"]
pub(super) mod object;
#[path = "blob/parts.rs"]
mod parts;
#[path = "blob/slice.rs"]
mod slice;

pub(super) fn bytes(value: &JsValue) -> Option<Vec<u8>> {
    access::bytes(value)
}

pub(super) fn mime_type(value: &JsValue) -> String {
    access::mime_type(value)
}

pub(super) fn named_value(value: &JsValue, name: String) -> JsValue {
    access::named_value(value, name)
}

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "Blob".into(),
        native("Blob", None, move |args| {
            Ok(object::blob_object(
                parts::collect(args.first()),
                parts::option_type(args.get(1)),
            ))
        }),
    );
    window.insert(
        "File".into(),
        native("File", None, move |args| {
            Ok(object::file_object(
                parts::collect(args.first()),
                args.get(1)
                    .map(JsValue::display)
                    .unwrap_or_else(|| "blob".into()),
                parts::option_type(args.get(2)),
                parts::option_last_modified(args.get(2)),
            ))
        }),
    );
}
