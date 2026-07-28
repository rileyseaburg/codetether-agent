use super::*;

pub(super) fn filter_result(filter: &JsValue, node: JsValue) -> Result<i32, String> {
    let call = match filter {
        JsValue::Undefined | JsValue::Null => None,
        JsValue::Function(_) | JsValue::Native(_) | JsValue::BoundFunction(_) => {
            Some((filter.clone(), JsValue::Undefined))
        }
        JsValue::Object(obj) => obj
            .borrow()
            .get("acceptNode")
            .cloned()
            .map(|method| (method, filter.clone())),
        _ => None,
    };
    let Some((callee, this_value)) = call else {
        return Ok(FILTER_ACCEPT);
    };
    let value = js::call_function_with_this(callee, this_value, &[node])?;
    Ok(filter_code(&value))
}

fn filter_code(value: &JsValue) -> i32 {
    let code = match value {
        JsValue::Number(n) if n.is_finite() => *n as i32,
        value => value.display().parse().unwrap_or(FILTER_ACCEPT),
    };
    match code {
        FILTER_REJECT | FILTER_SKIP => code,
        _ => FILTER_ACCEPT,
    }
}
