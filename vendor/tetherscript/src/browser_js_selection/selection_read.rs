use super::*;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    object.borrow_mut().insert(
        "toString".into(),
        native("Selection.toString", Some(0), move |_| {
            Ok(JsValue::String(props::selection_text()))
        }),
    );
    object.borrow_mut().insert(
        "getRangeAt".into(),
        native("Selection.getRangeAt", Some(1), move |args| {
            let index = args.first().map(selection_index).unwrap_or(usize::MAX);
            Ok(if index == 0 {
                registry::selection()
                    .map(range_object::object)
                    .unwrap_or(JsValue::Null)
            } else {
                JsValue::Null
            })
        }),
    );
}
