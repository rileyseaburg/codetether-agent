use super::super::super::{JsValue, NativeFunction};
use super::AwaitResult;
use std::rc::Rc;

pub(super) fn pair(done: AwaitResult) -> [JsValue; 2] {
    [one(done.clone(), true), one(done, false)]
}

fn one(done: AwaitResult, ok: bool) -> JsValue {
    let name = if ok { "await.resolve" } else { "await.reject" };
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(1), move |args| {
        let value = args.first().cloned().unwrap_or(JsValue::Undefined);
        let result = if ok { Ok(value) } else { Err(value.display()) };
        *done.borrow_mut() = Some(result);
        Ok(JsValue::Undefined)
    })))
}
