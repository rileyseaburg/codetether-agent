use super::*;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    strings: model::SharedStrings,
    types: model::SharedArray,
) {
    object.insert(
        "setData".into(),
        native("DataTransfer.setData", Some(2), move |args| {
            let mut strings = strings.borrow_mut();
            model::set_string(
                &mut strings,
                model::string_arg(args, 0),
                model::string_arg(args, 1),
            );
            model::sync_types(&types, &strings);
            Ok(JsValue::Undefined)
        }),
    );
}
