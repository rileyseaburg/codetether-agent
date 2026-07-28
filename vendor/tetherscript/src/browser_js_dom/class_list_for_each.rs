use super::*;

pub(super) fn run(
    handle: &DomHandle,
    weak: &ListWeak,
    args: &[JsValue],
) -> Result<JsValue, String> {
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "classList.forEach: expected callback".to_string())?;
    let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    let this_list = weak
        .upgrade()
        .map(JsValue::Object)
        .unwrap_or(JsValue::Undefined);
    for (index, token) in tokens::current(handle).into_iter().enumerate() {
        js::call_function_with_this(
            callback.clone(),
            this_arg.clone(),
            &[
                JsValue::String(token),
                JsValue::Number(index as f64),
                this_list.clone(),
            ],
        )?;
    }
    Ok(JsValue::Undefined)
}
