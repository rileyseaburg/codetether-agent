use super::*;

type ItemEntry = (String, JsValue);

pub(super) fn construct(args: &[JsValue]) -> Result<JsValue, String> {
    let Some(data) = args.first() else {
        return Err("ClipboardItem: expected data object".into());
    };
    Ok(create(entries(data)?))
}

fn entries(data: &JsValue) -> Result<Vec<ItemEntry>, String> {
    let JsValue::Object(object) = data else {
        return Err(format!(
            "ClipboardItem: expected data object, got {}",
            data.display()
        ));
    };
    let mut entries = object
        .borrow()
        .iter()
        .map(|(name, item)| (name.clone(), value::blob_for(name, item)))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(entries)
}

fn create(entries: Vec<ItemEntry>) -> JsValue {
    let lookup = entries.clone();
    let mut object = HashMap::new();
    object.insert("types".into(), types_array(&entries));
    object.insert(
        "getType".into(),
        native("ClipboardItem.getType", Some(1), move |args| {
            let requested = args.first().map(JsValue::display).unwrap_or_default();
            match lookup.iter().find(|(name, _)| name == &requested) {
                Some((_, value)) => Ok(promise::api::fulfilled(value.clone())),
                None => Ok(reject::promise(&requested)),
            }
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn types_array(entries: &[ItemEntry]) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        entries
            .iter()
            .map(|(name, _)| JsValue::String(name.clone()))
            .collect(),
    )))
}
