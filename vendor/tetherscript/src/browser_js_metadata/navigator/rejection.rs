use crate::js::{self, JsValue, NativeFunction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) fn thenable(reason: JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert("__promise_state".into(), JsValue::String("rejected".into()));
    object.insert("__promise_reason".into(), reason.clone());
    object.insert("reason".into(), reason.clone());
    object.insert("then".into(), then_method(reason.clone()));
    object.insert("catch".into(), catch_method(reason));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn then_method(reason: JsValue) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        "Promise.then",
        None,
        move |args| {
            let handler = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            settle(handler, reason.clone())
        },
    )))
}

fn catch_method(reason: JsValue) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        "Promise.catch",
        None,
        move |args| {
            let handler = args.first().cloned().unwrap_or(JsValue::Undefined);
            settle(handler, reason.clone())
        },
    )))
}

fn settle(handler: JsValue, reason: JsValue) -> Result<JsValue, String> {
    if matches!(handler, JsValue::Undefined | JsValue::Null) {
        return Ok(thenable(reason));
    }
    let value = js::call_function_with_this(handler, JsValue::Undefined, &[reason])?;
    Ok(super::thenable::resolved(value))
}
