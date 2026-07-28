use super::*;

pub(super) fn method(
    name: &'static str,
    owner: &Rc<RefCell<HashMap<String, JsValue>>>,
    keys: &'static [&'static str],
) -> JsValue {
    let weak = Rc::downgrade(owner);
    native(name, Some(0), move |_| Ok(snapshot(&weak, keys)))
}

fn snapshot(owner: &Weak<RefCell<HashMap<String, JsValue>>>, keys: &[&'static str]) -> JsValue {
    let Some(owner) = owner.upgrade() else {
        return JsValue::Object(Rc::new(RefCell::new(HashMap::new())));
    };
    let source = owner.borrow();
    let values = keys.iter().map(|key| entry(&source, key)).collect();
    JsValue::Object(Rc::new(RefCell::new(values)))
}

fn entry(source: &HashMap<String, JsValue>, key: &str) -> (String, JsValue) {
    (
        key.into(),
        source.get(key).cloned().unwrap_or(JsValue::Undefined),
    )
}
