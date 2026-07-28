use super::*;

pub(super) fn object(files: Vec<AgentFile>) -> JsValue {
    let values = files.iter().map(file_object::object).collect::<Vec<_>>();
    let obj = Rc::new(RefCell::new(HashMap::new()));
    let this_value = JsValue::Object(obj.clone());
    {
        let mut obj = obj.borrow_mut();
        obj.insert("length".into(), JsValue::Number(values.len() as f64));
        for (index, value) in values.iter().enumerate() {
            obj.insert(index.to_string(), value.clone());
        }
        install_item(&mut obj, values.clone());
        list_rows::install(&mut obj, values.clone());
        list_each::install(&mut obj, values, this_value.clone());
    }
    this_value
}

fn install_item(obj: &mut HashMap<String, JsValue>, values: Vec<JsValue>) {
    obj.insert(
        "item".into(),
        native("FileList.item", Some(1), move |args| {
            Ok(values
                .get(index(args.first()))
                .cloned()
                .unwrap_or(JsValue::Null))
        }),
    );
}

fn index(value: Option<&JsValue>) -> usize {
    match value {
        Some(JsValue::Number(n)) if n.is_finite() && *n >= 0.0 => n.trunc() as usize,
        Some(value) => value.display().parse().unwrap_or(usize::MAX),
        None => usize::MAX,
    }
}
