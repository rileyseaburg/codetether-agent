use super::super::*;

#[path = "legacy_collect.rs"]
mod collect;
#[path = "legacy_match.rs"]
mod matcher;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    install_method(obj, handle, "getElementsByTagName", collect::by_tag);
    install_method(obj, handle, "getElementsByClassName", collect::by_class);
    install_method(obj, handle, "getElementsByName", collect::by_name);
    obj.insert(
        "execCommand".into(),
        native("document.execCommand", None, |_| Ok(JsValue::Bool(false))),
    );
    obj.insert(
        "queryCommandSupported".into(),
        native("document.queryCommandSupported", None, |_| {
            Ok(JsValue::Bool(false))
        }),
    );
    obj.insert(
        "queryCommandEnabled".into(),
        native("document.queryCommandEnabled", None, |_| {
            Ok(JsValue::Bool(false))
        }),
    );
}

fn install_method(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    name: &'static str,
    collect: fn(&DomHandle, &str) -> JsValue,
) {
    let h = handle.clone();
    obj.insert(
        name.into(),
        native(name, Some(1), move |args| {
            let query = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(collect(&h, &query))
        }),
    );
}
