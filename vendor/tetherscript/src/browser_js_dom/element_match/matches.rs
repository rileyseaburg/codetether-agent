use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    for name in ["matches", "webkitMatchesSelector", "msMatchesSelector"] {
        install_one(obj, handle, name);
    }
}

fn install_one(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, name: &'static str) {
    let h = handle.clone();
    obj.insert(
        name.into(),
        native(name, Some(1), move |args| {
            Ok(JsValue::Bool(is_match(&h, &selector_arg(args))))
        }),
    );
}

fn is_match(handle: &DomHandle, selector: &str) -> bool {
    selector_paths(handle, selector)
        .iter()
        .any(|path| path == &handle.path)
}
