use super::*;

pub(super) fn install(object: &mut HashMap<String, JsValue>, entries: model::SharedEntries) {
    add(object, "entries", entries.clone(), entry_rows);
    add(object, "keys", entries.clone(), key_rows);
    add(object, "values", entries, value_rows);
}

fn add(
    object: &mut HashMap<String, JsValue>,
    method: &'static str,
    entries: model::SharedEntries,
    build: fn(&[model::FormEntry]) -> JsValue,
) {
    let native_name = format!("FormData.{method}");
    object.insert(
        method.into(),
        native(&native_name, Some(0), move |_| Ok(build(&entries.borrow()))),
    );
}

pub(super) fn entry_rows(entries: &[model::FormEntry]) -> JsValue {
    array(
        entries
            .iter()
            .map(|entry| {
                JsValue::Array(Rc::new(RefCell::new(vec![
                    JsValue::String(entry.name.clone()),
                    entry.value.clone(),
                ])))
            })
            .collect(),
    )
}

fn key_rows(entries: &[model::FormEntry]) -> JsValue {
    array(
        entries
            .iter()
            .map(|entry| JsValue::String(entry.name.clone()))
            .collect(),
    )
}

fn value_rows(entries: &[model::FormEntry]) -> JsValue {
    array(entries.iter().map(|entry| entry.value.clone()).collect())
}

fn array(items: Vec<JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(items)))
}
