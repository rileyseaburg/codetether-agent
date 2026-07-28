use super::*;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "add".into(),
        native("classList.add", None, move |args| add(&h, &list, args)),
    );
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "remove".into(),
        native("classList.remove", None, move |args| {
            remove(&h, &list, args)
        }),
    );
}

fn add(handle: &DomHandle, weak: &ListWeak, args: &[JsValue]) -> Result<JsValue, String> {
    let add = validate::all(args, "add")?;
    let mut tokens = tokens::current(handle);
    for token in add {
        if !tokens.iter().any(|item| item == &token) {
            tokens.push(token);
        }
    }
    commit(handle, weak, tokens);
    Ok(JsValue::Undefined)
}

fn remove(handle: &DomHandle, weak: &ListWeak, args: &[JsValue]) -> Result<JsValue, String> {
    let remove = validate::all(args, "remove")?;
    let tokens = tokens::current(handle)
        .into_iter()
        .filter(|token| !remove.iter().any(|item| item == token))
        .collect();
    commit(handle, weak, tokens);
    Ok(JsValue::Undefined)
}
