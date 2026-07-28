use super::*;

pub(super) fn blob_for(requested_type: &str, value: &JsValue) -> JsValue {
    if blob::bytes(value).is_some() {
        value.clone()
    } else {
        blob_object(value.display().into_bytes(), requested_type.into())
    }
}

fn blob_object(data: Vec<u8>, mime_type: String) -> JsValue {
    let mut object = HashMap::new();
    object.insert("size".into(), JsValue::Number(data.len() as f64));
    object.insert("type".into(), JsValue::String(mime_type.clone()));
    object.insert("__blobBytes".into(), bytes::byte_array(data.clone()));
    object.insert("__blobType".into(), JsValue::String(mime_type));
    install_methods(&mut object, data);
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn install_methods(object: &mut HashMap<String, JsValue>, data: Vec<u8>) {
    let text_data = data.clone();
    object.insert(
        "text".into(),
        native("ClipboardItem.Blob.text", Some(0), move |_| {
            Ok(promise::api::fulfilled(JsValue::String(
                String::from_utf8_lossy(&text_data).into(),
            )))
        }),
    );
    let buffer_data = data.clone();
    object.insert(
        "arrayBuffer".into(),
        native("ClipboardItem.Blob.arrayBuffer", Some(0), move |_| {
            Ok(promise::api::fulfilled(bytes::byte_array(
                buffer_data.clone(),
            )))
        }),
    );
    object.insert(
        "bytes".into(),
        native("ClipboardItem.Blob.bytes", Some(0), move |_| {
            Ok(promise::api::fulfilled(bytes::byte_array(data.clone())))
        }),
    );
}
