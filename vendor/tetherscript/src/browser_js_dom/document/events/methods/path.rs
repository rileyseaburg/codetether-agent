use super::*;

pub(super) fn install(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    event.borrow_mut().insert(
        "composedPath".into(),
        native("Event.composedPath", Some(0), move |_| {
            Ok(JsValue::Array(Rc::new(RefCell::new(Vec::new()))))
        }),
    );
}
