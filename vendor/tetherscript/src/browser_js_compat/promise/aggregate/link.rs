use super::super::*;

pub(super) fn attach_pending(value: &JsValue, ok: JsValue, err: JsValue) -> Result<bool, String> {
    if !matches!(state::from_value(value), Some(state::PromiseState::Pending)) {
        return Ok(false);
    }
    let JsValue::Object(object) = value else {
        return Ok(false);
    };
    let Some(then) = object.borrow().get("then").cloned() else {
        return Ok(false);
    };
    js::call_function_with_this(then, value.clone(), &[ok, err])?;
    Ok(true)
}
