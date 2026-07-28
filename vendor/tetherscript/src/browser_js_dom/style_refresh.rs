use super::*;

pub(super) fn update(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &handle_ref::HandleRef) {
    let current = handle.current();
    let names = style_attr::style_order::names(&style_attr::raw(&current));
    let mut obj = obj.borrow_mut();
    obj.retain(|key, _| key.parse::<usize>().is_err());
    obj.insert("length".into(), JsValue::Number(names.len() as f64));
    obj.insert(
        "cssText".into(),
        JsValue::String(style_attr::css_text(&current)),
    );
    for (index, name) in names.into_iter().enumerate() {
        obj.insert(index.to_string(), JsValue::String(name));
    }
}

pub(super) fn item(handle: &handle_ref::HandleRef, index: usize) -> String {
    style_attr::style_order::names(&style_attr::raw(&handle.current()))
        .get(index)
        .cloned()
        .unwrap_or_default()
}
