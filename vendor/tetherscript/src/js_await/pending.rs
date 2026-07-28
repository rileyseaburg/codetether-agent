use super::super::{call_with_this, get_property, is_callable, JsValue};
use super::{drain_once, field, state};
use std::cell::RefCell;
use std::rc::Rc;

#[path = "pending/settler.rs"]
mod settler;

const LIMIT: usize = 1000;
type AwaitResult = Rc<RefCell<Option<Result<JsValue, String>>>>;

pub(super) fn wait(value: JsValue) -> Result<JsValue, String> {
    let done = Rc::new(RefCell::new(None));
    let then = get_property(&value, "then")?;
    if !is_callable(&then) {
        return Ok(value);
    }
    call_with_this(then, value.clone(), &settler::pair(done.clone()))?;
    for _ in 0..LIMIT {
        if let Some(result) = take(&done) {
            return result;
        }
        if let Some(result) = settled(&value) {
            return result;
        }
        if !drain_once()? {
            break;
        }
    }
    take(&done).or_else(|| settled(&value)).unwrap_or(Ok(value))
}

fn take(done: &AwaitResult) -> Option<Result<JsValue, String>> {
    done.borrow_mut().take()
}

fn settled(value: &JsValue) -> Option<Result<JsValue, String>> {
    match state(value).as_deref()? {
        "fulfilled" => Some(Ok(field(value, &["__promise_value", "value"]))),
        "rejected" => Some(Err(field(value, &["__promise_reason", "reason"]).display())),
        _ => None,
    }
}
