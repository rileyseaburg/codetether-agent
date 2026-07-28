use crate::js::{self, JsValue, NativeFunction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) fn resolved(value: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    object.insert("__promise_value".into(), value.clone());
    object.insert("then".into(), then_method(value.clone()));
    object.insert("catch".into(), catch_method(value));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn then_method(value: JsValue) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        "Promise.then",
        None,
        move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            if matches!(callback, JsValue::Undefined | JsValue::Null) {
                return Ok(resolved(value.clone()));
            }
            let result = js::call_function_with_this(
                callback,
                JsValue::Undefined,
                std::slice::from_ref(&value),
            )?;
            Ok(resolved(result))
        },
    )))
}

fn catch_method(value: JsValue) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        "Promise.catch",
        None,
        move |_| Ok(resolved(value.clone())),
    )))
}
