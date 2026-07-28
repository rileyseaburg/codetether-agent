use super::*;

pub(super) fn resolved(value: JsValue) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let out = JsValue::Object(object.clone());
    let then_value = value;
    let then_out = out.clone();
    object.borrow_mut().insert(
        "then".into(),
        native("media.then", None, move |args| {
            call(args.first(), &then_value)?;
            Ok(then_out.clone())
        }),
    );
    let catch_out = out.clone();
    object.borrow_mut().insert(
        "catch".into(),
        native("media.catch", None, move |_| Ok(catch_out.clone())),
    );
    out
}

pub(super) fn rejected(error: JsValue) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let out = JsValue::Object(object.clone());
    let then_out = out.clone();
    object.borrow_mut().insert(
        "then".into(),
        native("media.then", None, move |_| Ok(then_out.clone())),
    );
    let catch_error = error;
    let catch_out = out.clone();
    object.borrow_mut().insert(
        "catch".into(),
        native("media.catch", None, move |args| {
            call(args.first(), &catch_error)?;
            Ok(catch_out.clone())
        }),
    );
    out
}

fn call(callback: Option<&JsValue>, value: &JsValue) -> Result<(), String> {
    if let Some(callback) = callback.filter(|callback| callback.truthy()) {
        js::call_function_with_this(
            callback.clone(),
            JsValue::Undefined,
            std::slice::from_ref(value),
        )?;
    }
    Ok(())
}
