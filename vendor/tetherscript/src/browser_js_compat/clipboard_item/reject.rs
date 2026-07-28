use super::*;

pub(super) fn promise(requested: &str) -> JsValue {
    from_reason(JsValue::String(format!(
        "ClipboardItem: missing type {}",
        requested
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
    native("ClipboardItem.getType.then", None, move |args| {
        let Some(handler) = args.get(1) else {
            return Ok(from_reason(reason.clone()));
        };
        handle(handler.clone(), reason.clone())
    })
}

fn catch_method(reason: JsValue) -> JsValue {
    native("ClipboardItem.getType.catch", None, move |args| {
        let Some(handler) = args.first() else {
            return Ok(from_reason(reason.clone()));
        };
        handle(handler.clone(), reason.clone())
    })
}

fn handle(handler: JsValue, reason: JsValue) -> Result<JsValue, String> {
    if matches!(handler, JsValue::Undefined | JsValue::Null) {
        return Ok(from_reason(reason));
    }
    match js::call_function_with_this(handler, JsValue::Undefined, &[reason]) {
        Ok(value) => Ok(promise::api::fulfilled(value)),
        Err(error) => Ok(from_reason(JsValue::String(error))),
    }
}
