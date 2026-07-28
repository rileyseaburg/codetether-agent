use super::*;

pub(super) fn install(object: &mut HashMap<String, JsValue>, entries: model::SharedEntries) {
    let for_append = entries.clone();
    object.insert(
        "append".into(),
        native("FormData.append", None, move |args| {
            for_append.borrow_mut().push(model::FormEntry {
                name: model::name_arg(args),
                value: model::value_for_append(args),
            });
            Ok(JsValue::Undefined)
        }),
    );
    let for_set = entries.clone();
    object.insert(
        "set".into(),
        native("FormData.set", None, move |args| {
            let name = model::name_arg(args);
            let value = model::value_for_append(args);
            let mut entries = for_set.borrow_mut();
            if let Some(pos) = entries.iter().position(|entry| entry.name == name) {
                entries[pos].value = value;
                let mut kept = false;
                entries.retain(|entry| {
                    if entry.name != name {
                        return true;
                    }
                    if kept {
                        false
                    } else {
                        kept = true;
                        true
                    }
                });
            } else {
                entries.push(model::FormEntry { name, value });
            }
            Ok(JsValue::Undefined)
        }),
    );
    object.insert(
        "delete".into(),
        native("FormData.delete", Some(1), move |args| {
            let name = model::name_arg(args);
            entries.borrow_mut().retain(|entry| entry.name != name);
            Ok(JsValue::Undefined)
        }),
    );
}
