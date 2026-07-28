use super::*;

pub(super) fn refresh_weak(obj: &ListWeak, handle: &DomHandle) {
    if let Some(obj) = obj.upgrade() {
        object(&obj, handle);
    }
}

pub(super) fn object(obj: &ListObject, handle: &DomHandle) {
    let tokens = tokens::current(handle);
    let mut obj = obj.borrow_mut();
    let stale = obj
        .keys()
        .filter(|key| index_key(key).is_some())
        .cloned()
        .collect::<Vec<_>>();
    for key in stale {
        obj.remove(&key);
    }
    obj.insert("length".into(), JsValue::Number(tokens.len() as f64));
    obj.insert("value".into(), JsValue::String(tokens.join(" ")));
    for (index, token) in tokens.iter().enumerate() {
        obj.insert(index.to_string(), JsValue::String(token.clone()));
    }
}

pub(super) fn install_value_setter(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "__set:value".into(),
        native("set_classList_value", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            tokens::set(&h, tokens::parse(&value));
            refresh_weak(&list, &h);
            Ok(JsValue::String(tokens::current(&h).join(" ")))
        }),
    );
}

fn index_key(key: &str) -> Option<usize> {
    key.parse::<usize>()
        .ok()
        .filter(|index| index.to_string() == key)
}
