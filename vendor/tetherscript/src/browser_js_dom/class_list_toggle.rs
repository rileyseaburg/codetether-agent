use super::*;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "toggle".into(),
        native("classList.toggle", None, move |args| run(&h, &list, args)),
    );
}

fn run(handle: &DomHandle, weak: &ListWeak, args: &[JsValue]) -> Result<JsValue, String> {
    let token = validate::required(args, 0, "toggle")?;
    let force = args.get(1).map(JsValue::truthy);
    let mut tokens = tokens::current(handle);
    let present = tokens.iter().any(|item| item == &token);
    let should_have = force.unwrap_or(!present);
    if should_have && !present {
        tokens.push(token.clone());
    } else if !should_have {
        tokens.retain(|item| item != &token);
    }
    commit(handle, weak, tokens);
    Ok(JsValue::Bool(should_have))
}
