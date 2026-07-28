use super::*;

pub(super) fn rejected(reason: &str) -> JsValue {
    rejected_value(JsValue::String(reason.into()))
}

fn rejected_value(reason: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert("__promise_state".into(), JsValue::String("rejected".into()));
    object.insert("__promise_reason".into(), reason.clone());
    object.insert("reason".into(), reason.clone());
    object.insert("then".into(), then_method(reason.clone()));
    object.insert("catch".into(), catch_method(reason));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn fulfilled(value: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    object.insert("__promise_value".into(), value.clone());
    install_then_catch_simple(&mut object, value);
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn then_method(reason: JsValue) -> JsValue {
    native("screen.orientation.lock.then", None, move |args| {
        settle(args.get(1).cloned(), reason.clone())
    })
}

fn catch_method(reason: JsValue) -> JsValue {
    native("screen.orientation.lock.catch", None, move |args| {
        settle(args.first().cloned(), reason.clone())
    })
}

fn settle(handler: Option<JsValue>, reason: JsValue) -> Result<JsValue, String> {
    let handler = handler.unwrap_or(JsValue::Undefined);
    if matches!(handler, JsValue::Undefined | JsValue::Null) {
        return Ok(rejected_value(reason));
    }
    Ok(fulfilled(js::call_function_with_this(
        handler,
        JsValue::Undefined,
        &[reason],
    )?))
}
