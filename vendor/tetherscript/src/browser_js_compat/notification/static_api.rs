use super::*;

pub(super) fn request_permission() -> JsValue {
    native("Notification.requestPermission", None, |args| {
        let permission = JsValue::String(PERMISSION.into());
        if let Some(callback) = args.first() {
            if !matches!(callback, JsValue::Undefined | JsValue::Null) {
                js::call_function_with_this(
                    callback.clone(),
                    JsValue::Undefined,
                    std::slice::from_ref(&permission),
                )?;
            }
        }
        Ok(promise::api::fulfilled(permission))
    })
}
