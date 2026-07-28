use super::*;

pub(super) type ItemObject = Rc<RefCell<HashMap<String, JsValue>>>;
pub(super) type SharedItems = Rc<RefCell<Vec<JsValue>>>;

pub(super) fn create() -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let items = Rc::new(RefCell::new(Vec::new()));
    items_methods::install(&object, &items);
    sync(&object, &items);
    JsValue::Object(object)
}

pub(super) fn sync(object: &ItemObject, items: &SharedItems) {
    let items = items.borrow();
    let mut object = object.borrow_mut();
    object.retain(|name, _| name.parse::<usize>().is_err());
    object.insert("length".into(), JsValue::Number(items.len() as f64));
    for (index, item) in items.iter().enumerate() {
        object.insert(index.to_string(), item.clone());
    }
}

pub(super) fn item(value: JsValue, type_name: String, kind: &str) -> JsValue {
    let mut item = HashMap::new();
    item.insert("kind".into(), JsValue::String(kind.into()));
    item.insert("type".into(), JsValue::String(type_name));
    item.insert("value".into(), value);
    JsValue::Object(Rc::new(RefCell::new(item)))
}

pub(super) fn index_arg(args: &[JsValue]) -> Option<usize> {
    let value = args.first()?.display().parse::<f64>().ok()?;
    if value.is_nan() || value < 0.0 {
        return None;
    }
    Some(value.trunc() as usize)
}
