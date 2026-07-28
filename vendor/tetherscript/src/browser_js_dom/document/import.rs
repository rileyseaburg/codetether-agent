use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    install_import(obj);
    install_adopt(obj);
}

fn install_import(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "importNode".into(),
        native("document.importNode", None, move |args| {
            let deep = args.get(1).is_some_and(JsValue::truthy);
            let value = args.first().unwrap_or(&JsValue::Undefined);
            Ok(ops::detached_object(ops::clone_for_import(value, deep)))
        }),
    );
}

fn install_adopt(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "adoptNode".into(),
        native("document.adoptNode", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined);
            let node = js_value_to_node(value);
            if let Some(handle) = dom_handle_from_value(value) {
                if !handle.path.is_empty() {
                    handle.remove_self()?;
                }
            }
            Ok(ops::detached_object(node))
        }),
    );
}
