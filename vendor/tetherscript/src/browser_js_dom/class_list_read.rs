use super::*;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "contains".into(),
        native("classList.contains", Some(1), move |args| {
            let token = validate::token(&args[0], "contains")?;
            Ok(JsValue::Bool(
                tokens::current(&h).iter().any(|t| t == &token),
            ))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "item".into(),
        native("classList.item", Some(1), move |args| {
            Ok(tokens::index(args.first())
                .and_then(|index| tokens::current(&h).get(index).cloned())
                .map(JsValue::String)
                .unwrap_or(JsValue::Null))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "toString".into(),
        native("classList.toString", Some(0), move |_| {
            Ok(JsValue::String(tokens::current(&h).join(" ")))
        }),
    );
}
