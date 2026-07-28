use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "closest".into(),
        native("closest", Some(1), move |args| {
            Ok(nearest(&h, &selector_arg(args))
                .map(node_object)
                .unwrap_or(JsValue::Null))
        }),
    );
}

fn nearest(handle: &DomHandle, selector: &str) -> Option<DomHandle> {
    let matches = selector_paths(handle, selector);
    let mut path = handle.path.clone();
    loop {
        if matches.iter().any(|candidate| candidate == &path) {
            return Some(DomHandle {
                root: handle.root.clone(),
                path,
            });
        }
        path.pop()?;
    }
}
