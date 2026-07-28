use super::super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, select: &DomHandle) {
    let h = select.clone();
    obj.borrow_mut().insert(
        "item".into(),
        native("HTMLOptionsCollection.item", Some(1), move |args| {
            Ok(item(&h, index_arg(args.first())))
        }),
    );
    let h = select.clone();
    obj.borrow_mut().insert(
        "namedItem".into(),
        native("HTMLOptionsCollection.namedItem", Some(1), move |args| {
            Ok(named(&h, &args[0].display()))
        }),
    );
}

fn item(select: &DomHandle, index: usize) -> JsValue {
    super::handles::all(select)
        .get(index)
        .map(|handle| ops::handle_object(handle.root.clone(), handle.path.clone()))
        .unwrap_or(JsValue::Null)
}

fn named(select: &DomHandle, name: &str) -> JsValue {
    super::handles::all(select)
        .into_iter()
        .find(|handle| names::matches(handle, name))
        .map(|handle| ops::handle_object(handle.root.clone(), handle.path.clone()))
        .unwrap_or(JsValue::Null)
}

fn index_arg(value: Option<&JsValue>) -> usize {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => n.trunc() as usize,
        other => other.display().parse().unwrap_or(usize::MAX),
    }
}
