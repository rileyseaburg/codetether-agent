use super::*;

pub(super) fn install(object: &mut HashMap<String, JsValue>) {
    object.insert("onchange".into(), JsValue::Null);
    object.insert(
        "addEventListener".into(),
        native("CookieStore.addEventListener", Some(2), |_| {
            Ok(JsValue::Undefined)
        }),
    );
    object.insert(
        "removeEventListener".into(),
        native("CookieStore.removeEventListener", Some(2), |_| {
            Ok(JsValue::Undefined)
        }),
    );
    object.insert(
        "dispatchEvent".into(),
        native("CookieStore.dispatchEvent", Some(1), |_| {
            Ok(JsValue::Bool(true))
        }),
    );
}
