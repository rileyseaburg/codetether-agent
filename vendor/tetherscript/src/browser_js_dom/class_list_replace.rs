use super::*;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    let h = handle.clone();
    let list = weak.clone();
    obj.insert(
        "replace".into(),
        native("classList.replace", Some(2), move |args| {
            run(&h, &list, args)
        }),
    );
}

fn run(handle: &DomHandle, weak: &ListWeak, args: &[JsValue]) -> Result<JsValue, String> {
    let old = validate::required(args, 0, "replace")?;
    let new = validate::required(args, 1, "replace")?;
    let mut tokens = tokens::current(handle);
    let Some(index) = tokens.iter().position(|item| item == &old) else {
        return Ok(JsValue::Bool(false));
    };
    if old != new {
        if tokens.iter().any(|item| item == &new) {
            tokens.remove(index);
        } else {
            tokens[index] = new;
        }
        commit(handle, weak, tokens);
    }
    Ok(JsValue::Bool(true))
}
