use super::*;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    strings: model::SharedStrings,
    types: model::SharedArray,
) {
    object.insert(
        "clearData".into(),
        native("DataTransfer.clearData", None, move |args| {
            let mut strings = strings.borrow_mut();
            model::clear_string(&mut strings, args.first().map(JsValue::display));
            model::sync_types(&types, &strings);
            Ok(JsValue::Undefined)
        }),
    );
}
