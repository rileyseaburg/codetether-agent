use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    values: Vec<JsValue>,
    this_value: JsValue,
) {
    obj.insert(
        "forEach".into(),
        native("FileList.forEach", None, move |args| {
            let callback = required_callback(args)?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            call_each(&values, callback, this_arg, this_value.clone())
        }),
    );
}

fn required_callback(args: &[JsValue]) -> Result<JsValue, String> {
    args.first()
        .cloned()
        .ok_or_else(|| "FileList.forEach: expected callback".to_string())
}

fn call_each(
    values: &[JsValue],
    callback: JsValue,
    this_arg: JsValue,
    this_value: JsValue,
) -> Result<JsValue, String> {
    for (index, value) in values.iter().cloned().enumerate() {
        js::call_function_with_this(
            callback.clone(),
            this_arg.clone(),
            &[value, JsValue::Number(index as f64), this_value.clone()],
        )?;
    }
    Ok(JsValue::Undefined)
}
