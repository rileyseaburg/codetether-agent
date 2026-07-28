use super::*;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let add_object = object.clone();
    object.borrow_mut().insert(
        "addRange".into(),
        native("Selection.addRange", Some(1), move |args| {
            let range = args.first().and_then(registry::range_from_value);
            registry::set_selection(range.map(|range| range.borrow().clone()));
            props::refresh_selection(&add_object);
            Ok(JsValue::Undefined)
        }),
    );
    let clear_object = object.clone();
    object.borrow_mut().insert(
        "removeAllRanges".into(),
        native("Selection.removeAllRanges", Some(0), move |_| {
            registry::set_selection(None);
            props::refresh_selection(&clear_object);
            Ok(JsValue::Undefined)
        }),
    );
}
