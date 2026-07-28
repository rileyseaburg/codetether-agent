use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: handle_ref::HandleRef) {
    install_get(obj, &handle);
    install_set(obj, &handle);
    install_remove(obj, &handle);
    style_css_text::install(obj, handle);
}

fn install_get(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &handle_ref::HandleRef) {
    let h = handle.clone();
    obj.borrow_mut().insert(
        "getPropertyValue".into(),
        native(
            "CSSStyleDeclaration.getPropertyValue",
            Some(1),
            move |args| {
                let name = args.first().unwrap_or(&JsValue::Undefined).display();
                Ok(JsValue::String(style_attr::get(&h.current(), &name)))
            },
        ),
    );
}

fn install_set(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &handle_ref::HandleRef) {
    let h = handle.clone();
    let out = obj.clone();
    obj.borrow_mut().insert(
        "setProperty".into(),
        native("CSSStyleDeclaration.setProperty", Some(2), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
            style_attr::set_prop(&h.current(), &name, value)?;
            style_refresh::update(&out, &h);
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_remove(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &handle_ref::HandleRef) {
    let h = handle.clone();
    let out = obj.clone();
    obj.borrow_mut().insert(
        "removeProperty".into(),
        native("CSSStyleDeclaration.removeProperty", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let old = style_remove::remove_prop(&h.current(), &name)?;
            style_refresh::update(&out, &h);
            Ok(JsValue::String(old))
        }),
    );
}
