use super::*;

pub(super) fn install(object: &mut HashMap<String, JsValue>, strings: model::SharedStrings) {
    object.insert(
        "getData".into(),
        native("DataTransfer.getData", Some(1), move |args| {
            Ok(JsValue::String(model::get_string(
                &strings.borrow(),
                &model::string_arg(args, 0),
            )))
        }),
    );
}
