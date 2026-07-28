use super::*;

pub(super) fn promise(method: &str) -> JsValue {
    from_reason(JsValue::String(format!(
        "crypto.subtle.{method}: unsupported"
    )))
}

fn from_reason(reason: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert("__promise_state".into(), JsValue::String("rejected".into()));
    object.insert("__promise_reason".into(), reason.clone());
    object.insert("reason".into(), reason.clone());
    object.insert("then".into(), then_method(reason.clone()));
    object.insert("catch".into(), catch_method(reason));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn then_method(reason: JsValue) -> JsValue {
    native("crypto.subtle.rejected.then", None, move |args| {
        let handler = args.get(1).cloned().unwrap_or(JsValue::Undefined);
        handle(handler, reason.clone())
    })
}

fn catch_method(reason: JsValue) -> JsValue {
    native("crypto.subtle.rejected.catch", None, move |args| {
        let handler = args.first().cloned().unwrap_or(JsValue::Undefined);
        handle(handler, reason.clone())
    })
}

fn handle(handler: JsValue, reason: JsValue) -> Result<JsValue, String> {
    if matches!(handler, JsValue::Undefined | JsValue::Null) {
        return Ok(from_reason(reason));
    }
    let value = js::call_function_with_this(handler, JsValue::Undefined, &[reason])?;
    Ok(super::super::fulfilled_promise(value))
}
