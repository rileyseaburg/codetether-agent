use super::*;

pub(super) fn install(event: &JsValue, path: &[DomHandle]) {
    let values = path
        .iter()
        .rev()
        .map(|handle| node_object(handle.clone()))
        .collect::<Vec<_>>();
    let JsValue::Object(obj) = event else {
        return;
    };
    let path_values = values.clone();
    let path = JsValue::Array(Rc::new(RefCell::new(values)));
    obj.borrow_mut().insert("path".into(), path);
    obj.borrow_mut().insert(
        "composedPath".into(),
        native("Event.composedPath", Some(0), move |_| {
            Ok(JsValue::Array(Rc::new(RefCell::new(path_values.clone()))))
        }),
    );
}
