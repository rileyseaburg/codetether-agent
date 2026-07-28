use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: handle_ref::HandleRef) {
    obj.borrow_mut().insert(
        "item".into(),
        native("CSSStyleDeclaration.item", Some(1), move |args| {
            let index = args.first().map(index_value).unwrap_or_default();
            Ok(JsValue::String(style_refresh::item(&handle, index)))
        }),
    );
}

fn index_value(value: &JsValue) -> usize {
    match value {
        JsValue::Number(number) if number.is_finite() && *number > 0.0 => *number as usize,
        _ => value.display().parse().unwrap_or_default(),
    }
}
