use super::*;

pub(super) fn install_document(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    install_getter(obj, handle, "document.getSelection");
}

pub(super) fn install_window(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    install_getter(obj, handle, "window.getSelection");
}

fn install_getter(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, name: &'static str) {
    let root = handle.clone();
    obj.insert(
        "getSelection".into(),
        native(name, Some(0), move |_| Ok(object(&root))),
    );
}

fn object(_root: &DomHandle) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    props::refresh_selection(&object);
    selection_mutation::install(&object);
    selection_read::install(&object);
    JsValue::Object(object)
}
