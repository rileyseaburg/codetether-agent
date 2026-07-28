use super::*;

pub(super) type SharedEntries = Rc<RefCell<Vec<FormEntry>>>;

#[derive(Clone)]
pub(super) struct FormEntry {
    pub(super) name: String,
    pub(super) value: JsValue,
}

pub(super) fn from_arg(value: Option<&JsValue>) -> Vec<FormEntry> {
    let Some(value) = value else {
        return Vec::new();
    };
    let Some(handle) = dom_handle_from_value(value) else {
        return Vec::new();
    };
    collect_form_entries(&handle)
        .into_iter()
        .map(|(name, value)| FormEntry {
            name,
            value: JsValue::String(value),
        })
        .collect()
}

pub(super) fn value_for_append(args: &[JsValue]) -> JsValue {
    let value = args
        .get(1)
        .cloned()
        .unwrap_or(JsValue::String(String::new()));
    match args.get(2) {
        Some(name) if blob::bytes(&value).is_some() => blob::named_value(&value, name.display()),
        _ if blob::bytes(&value).is_some() => value,
        _ => JsValue::String(value.display()),
    }
}

pub(super) fn name_arg(args: &[JsValue]) -> String {
    args.first().map(JsValue::display).unwrap_or_default()
}
