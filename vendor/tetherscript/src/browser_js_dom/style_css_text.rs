use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: handle_ref::HandleRef) {
    let out = obj.clone();
    obj.borrow_mut().insert(
        "__set:cssText".into(),
        native("CSSStyleDeclaration.setCssText", Some(1), move |args| {
            let text = args.first().unwrap_or(&JsValue::Undefined).display();
            let text = style_attr::set_css_text(&handle.current(), text)?;
            style_refresh::update(&out, &handle);
            Ok(JsValue::String(text))
        }),
    );
}
