use super::*;

pub(super) fn install(document: &JsValue, root: Rc<RefCell<Node>>) {
    let JsValue::Object(object) = document else {
        return;
    };
    let single_root = root.clone();
    let all_root = root;
    let single_document = object.clone();
    let all_document = object.clone();
    let mut object = object.borrow_mut();
    object.insert("hidden".into(), JsValue::Bool(false));
    object.insert("visibilityState".into(), JsValue::String("visible".into()));
    object.insert(
        "elementFromPoint".into(),
        native("document.elementFromPoint", Some(2), move |args| {
            let (x, y) = point::from_args(args);
            let (doc_x, doc_y) = point::scrolled(&single_document, x, y);
            Ok(hit::single_at(&single_root, x, y, doc_x, doc_y))
        }),
    );
    object.insert(
        "elementsFromPoint".into(),
        native("document.elementsFromPoint", Some(2), move |args| {
            let (x, y) = point::from_args(args);
            let (doc_x, doc_y) = point::scrolled(&all_document, x, y);
            Ok(hit::all_at(&all_root, x, y, doc_x, doc_y))
        }),
    );
}
