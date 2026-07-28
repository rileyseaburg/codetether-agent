use super::*;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    entries: model::SharedEntries,
    this_value: JsValue,
) {
    object.insert(
        "forEach".into(),
        native("FormData.forEach", None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| "FormData.forEach: expected callback".to_string())?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            for entry in entries.borrow().clone() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[entry.value, JsValue::String(entry.name), this_value.clone()],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
}
