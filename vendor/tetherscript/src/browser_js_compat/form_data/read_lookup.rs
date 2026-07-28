use super::*;

pub(super) fn install(object: &mut HashMap<String, JsValue>, entries: model::SharedEntries) {
    let for_get = entries.clone();
    object.insert(
        "get".into(),
        native("FormData.get", Some(1), move |args| {
            let name = model::name_arg(args);
            Ok(for_get
                .borrow()
                .iter()
                .find(|entry| entry.name == name)
                .map(|entry| entry.value.clone())
                .unwrap_or(JsValue::Null))
        }),
    );
    let for_all = entries.clone();
    object.insert(
        "getAll".into(),
        native("FormData.getAll", Some(1), move |args| {
            Ok(JsValue::Array(Rc::new(RefCell::new(
                for_all
                    .borrow()
                    .iter()
                    .filter(|entry| entry.name == model::name_arg(args))
                    .map(|entry| entry.value.clone())
                    .collect(),
            ))))
        }),
    );
    object.insert(
        "has".into(),
        native("FormData.has", Some(1), move |args| {
            Ok(JsValue::Bool(
                entries
                    .borrow()
                    .iter()
                    .any(|entry| entry.name == model::name_arg(args)),
            ))
        }),
    );
}
