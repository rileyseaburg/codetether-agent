use super::*;

pub(super) fn fulfilled(value: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    object.insert("__promise_value".into(), value.clone());
    object.insert("value".into(), value.clone());
    install_then(&mut object, value);
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn install_then(object: &mut HashMap<String, JsValue>, value: JsValue) {
    let fulfilled_value = value.clone();
    object.insert(
        "then".into(),
        native("CookieStore.then", None, move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            if matches!(callback, JsValue::Undefined | JsValue::Null) {
                return Ok(fulfilled(fulfilled_value.clone()));
            }
            let next = js::call_function_with_this(
                callback,
                JsValue::Undefined,
                std::slice::from_ref(&fulfilled_value),
            )?;
            Ok(fulfilled(next))
        }),
    );
    object.insert(
        "catch".into(),
        native("CookieStore.catch", None, |_| {
            Ok(fulfilled(JsValue::Undefined))
        }),
    );
}
