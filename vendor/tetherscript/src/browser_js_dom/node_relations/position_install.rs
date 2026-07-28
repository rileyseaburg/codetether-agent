use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    let id = map_id(obj);
    obj.insert(
        "compareDocumentPosition".into(),
        native("Node.compareDocumentPosition", Some(1), move |args| {
            let other = args.first().unwrap_or(&JsValue::Undefined);
            Ok(JsValue::Number(
                position::compare(&handle.current(), &id, other) as f64,
            ))
        }),
    );
}

fn map_id(obj: &HashMap<String, JsValue>) -> String {
    match obj.get("__domHandleId") {
        Some(JsValue::String(id)) => id.clone(),
        _ => String::new(),
    }
}
