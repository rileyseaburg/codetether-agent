use super::*;

pub(super) fn sync(document: &JsValue) {
    if let JsValue::Object(object) = document {
        object
            .borrow_mut()
            .insert("cookie".into(), JsValue::String(state::cookie_string()));
    }
}
