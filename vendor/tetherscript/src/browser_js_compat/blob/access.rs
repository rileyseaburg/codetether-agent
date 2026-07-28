use super::*;

pub(super) fn bytes(value: &JsValue) -> Option<Vec<u8>> {
    property(value, "__blobBytes").map(|stored| bytes::bytes_from_value(&stored))
}

pub(super) fn mime_type(value: &JsValue) -> String {
    property(value, "__blobType")
        .or_else(|| property(value, "type"))
        .map(|value| value.display())
        .unwrap_or_default()
}

pub(super) fn named_value(value: &JsValue, name: String) -> JsValue {
    let data = bytes(value).unwrap_or_else(|| bytes::bytes_from_value(value));
    object::file_object(data, name, mime_type(value), 0.0)
}

fn property(value: &JsValue, name: &str) -> Option<JsValue> {
    let JsValue::Object(object) = value else {
        return None;
    };
    object.borrow().get(name).cloned()
}
