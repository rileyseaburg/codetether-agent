use super::*;

pub(super) type SharedArray = Rc<RefCell<Vec<JsValue>>>;
pub(super) type SharedStrings = Rc<RefCell<Vec<(String, String)>>>;

pub(super) fn array() -> SharedArray {
    Rc::new(RefCell::new(Vec::new()))
}

pub(super) fn string_arg(args: &[JsValue], index: usize) -> String {
    args.get(index).map(JsValue::display).unwrap_or_default()
}

pub(super) fn set_string(entries: &mut Vec<(String, String)>, name: String, value: String) {
    if let Some((_, old_value)) = entries.iter_mut().find(|(old_name, _)| old_name == &name) {
        *old_value = value;
    } else {
        entries.push((name, value));
    }
}

pub(super) fn get_string(entries: &[(String, String)], name: &str) -> String {
    entries
        .iter()
        .find(|(old_name, _)| old_name == name)
        .map(|(_, value)| value.clone())
        .unwrap_or_default()
}

pub(super) fn clear_string(entries: &mut Vec<(String, String)>, name: Option<String>) {
    if let Some(name) = name {
        entries.retain(|(old_name, _)| old_name != &name);
    } else {
        entries.clear();
    }
}

pub(super) fn sync_types(types: &SharedArray, entries: &[(String, String)]) {
    let mut types = types.borrow_mut();
    types.clear();
    types.extend(
        entries
            .iter()
            .map(|(name, _)| JsValue::String(name.clone())),
    );
}
