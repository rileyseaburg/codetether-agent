use super::*;

pub(super) fn set(document_object: JsValue) -> JsValue {
    native("CookieStore.set", None, move |args| {
        if let Some((name, value)) = options::set_args(args) {
            document::set_pair(&name, &value);
            projection::sync(&document_object);
        }
        Ok(thenable::fulfilled(JsValue::Undefined))
    })
}

pub(super) fn delete(document_object: JsValue) -> JsValue {
    native("CookieStore.delete", Some(1), move |args| {
        if let Some(name) = options::name(args.first()) {
            document::delete_pair(&name);
            projection::sync(&document_object);
        }
        Ok(thenable::fulfilled(JsValue::Undefined))
    })
}
