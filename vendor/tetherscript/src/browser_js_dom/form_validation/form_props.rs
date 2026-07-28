use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    obj.borrow_mut()
        .insert("elements".into(), elements_object::create(handle));
    sync_length(obj, handle);
    wrap_append_child(obj, handle);
}

fn wrap_append_child(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let Some(original) = obj.borrow().get("appendChild").cloned() else {
        return;
    };
    let h = handle.clone();
    let object = obj.clone();
    obj.borrow_mut().insert(
        "appendChild".into(),
        native("HTMLFormElement.appendChild", Some(1), move |args| {
            let this_value = JsValue::Object(object.clone());
            let result = js::call_function_with_this(original.clone(), this_value, args)?;
            sync_length(&object, &h);
            Ok(result)
        }),
    );
}

fn sync_length(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let len = match obj.borrow().get("elements").cloned() {
        Some(JsValue::Object(elements)) => elements_sync::write(&elements, handle),
        _ => 0,
    };
    obj.borrow_mut()
        .insert("length".into(), JsValue::Number(len as f64));
}
