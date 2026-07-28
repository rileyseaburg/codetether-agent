use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "XMLSerializer".into(),
        native("XMLSerializer", Some(0), move |_| Ok(serializer_object())),
    );
}

fn serializer_object() -> JsValue {
    let mut obj = HashMap::new();
    obj.insert(
        "serializeToString".into(),
        native("XMLSerializer.serializeToString", Some(1), move |args| {
            Ok(JsValue::String(ops::serialize_value(
                args.first().unwrap_or(&JsValue::Undefined),
            )))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
