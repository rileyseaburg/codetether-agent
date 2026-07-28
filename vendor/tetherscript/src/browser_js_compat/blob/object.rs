use super::*;

pub(super) fn blob_object(data: Vec<u8>, mime_type: String) -> JsValue {
    make_object(data, mime_type, None)
}

pub(super) fn file_object(
    data: Vec<u8>,
    name: String,
    mime_type: String,
    last_modified: f64,
) -> JsValue {
    make_object(data, mime_type, Some((name, last_modified)))
}

fn make_object(data: Vec<u8>, mime_type: String, file: Option<(String, f64)>) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut obj = object.borrow_mut();
        obj.insert("size".into(), JsValue::Number(data.len() as f64));
        obj.insert("type".into(), JsValue::String(mime_type.clone()));
        obj.insert("__blobBytes".into(), bytes::byte_array(data.clone()));
        obj.insert("__blobType".into(), JsValue::String(mime_type.clone()));
        if let Some((name, last_modified)) = file {
            obj.insert("name".into(), JsValue::String(name));
            obj.insert("lastModified".into(), JsValue::Number(last_modified));
            obj.insert("webkitRelativePath".into(), JsValue::String(String::new()));
        }
    }
    methods::install(&object, data, mime_type);
    JsValue::Object(object)
}
