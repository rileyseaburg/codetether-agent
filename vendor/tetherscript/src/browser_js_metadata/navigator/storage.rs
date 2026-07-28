use super::*;
use std::cell::RefCell;
use std::rc::Rc;

const QUOTA: f64 = 67_108_864.0;
const USAGE: f64 = 0.0;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    let persisted = Rc::new(RefCell::new(false));
    let mut storage = HashMap::new();
    estimate(&mut storage);
    persisted_method(&mut storage, persisted.clone());
    persist_method(&mut storage, persisted);
    navigator.insert(
        "storage".into(),
        JsValue::Object(Rc::new(RefCell::new(storage))),
    );
}

fn estimate(storage: &mut HashMap<String, JsValue>) {
    storage.insert(
        "estimate".into(),
        native("navigator.storage.estimate", Some(0), move |_| {
            Ok(thenable::resolved(estimate_object()))
        }),
    );
}

fn persisted_method(storage: &mut HashMap<String, JsValue>, persisted: Rc<RefCell<bool>>) {
    storage.insert(
        "persisted".into(),
        native("navigator.storage.persisted", Some(0), move |_| {
            Ok(thenable::resolved(JsValue::Bool(*persisted.borrow())))
        }),
    );
}

fn persist_method(storage: &mut HashMap<String, JsValue>, persisted: Rc<RefCell<bool>>) {
    storage.insert(
        "persist".into(),
        native("navigator.storage.persist", Some(0), move |_| {
            *persisted.borrow_mut() = true;
            Ok(thenable::resolved(JsValue::Bool(true)))
        }),
    );
}

fn estimate_object() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("quota".into(), JsValue::Number(QUOTA)),
        ("usage".into(), JsValue::Number(USAGE)),
    ]))))
}
