use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    data: Vec<u8>,
    mime_type: String,
) {
    let text_data = data.clone();
    object.borrow_mut().insert(
        "text".into(),
        native("Blob.text", Some(0), move |_| {
            Ok(promise::api::fulfilled(JsValue::String(
                String::from_utf8_lossy(&text_data).into(),
            )))
        }),
    );
    let buffer_data = data.clone();
    object.borrow_mut().insert(
        "arrayBuffer".into(),
        native("Blob.arrayBuffer", Some(0), move |_| {
            Ok(promise::api::fulfilled(bytes::byte_array(
                buffer_data.clone(),
            )))
        }),
    );
    let bytes_data = data.clone();
    object.borrow_mut().insert(
        "bytes".into(),
        native("Blob.bytes", Some(0), move |_| {
            Ok(promise::api::fulfilled(bytes::byte_array(
                bytes_data.clone(),
            )))
        }),
    );
    object.borrow_mut().insert(
        "slice".into(),
        native("Blob.slice", None, move |args| {
            let (start, end) = slice::range(args, data.len());
            let content_type = args
                .get(2)
                .map(|value| parts::normalize_type(&value.display()))
                .unwrap_or_default();
            let content_type = if content_type.is_empty() {
                mime_type.clone()
            } else {
                content_type
            };
            Ok(object::blob_object(data[start..end].to_vec(), content_type))
        }),
    );
}
