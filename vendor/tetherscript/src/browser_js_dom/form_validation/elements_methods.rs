use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let h = handle.clone();
    obj.borrow_mut().insert(
        "item".into(),
        native("HTMLFormControlsCollection.item", Some(1), move |args| {
            Ok(item(&h, index(args.first())))
        }),
    );
    let h = handle.clone();
    obj.borrow_mut().insert(
        "namedItem".into(),
        native(
            "HTMLFormControlsCollection.namedItem",
            Some(1),
            move |args| Ok(named(&h, &args[0].display())),
        ),
    );
}

fn item(handle: &DomHandle, index: usize) -> JsValue {
    listed::handles(handle)
        .get(index)
        .map(value)
        .unwrap_or(JsValue::Null)
}

fn named(handle: &DomHandle, name: &str) -> JsValue {
    listed::handles(handle)
        .into_iter()
        .find(|control| names::matches(control, name))
        .map(|control| value(&control))
        .unwrap_or(JsValue::Null)
}

fn value(handle: &DomHandle) -> JsValue {
    ops::handle_object(handle.root.clone(), handle.path.clone())
}

fn index(value: Option<&JsValue>) -> usize {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => n.trunc() as usize,
        other => other.display().parse().unwrap_or(usize::MAX),
    }
}
