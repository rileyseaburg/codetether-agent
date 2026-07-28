use super::super::*;

pub(super) fn call(
    definition: &JsValue,
    name: &str,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<(), String> {
    if let Some(callback) = util::callback(definition, name) {
        js::call_function_with_this(callback, this_value, args)?;
    }
    Ok(())
}
