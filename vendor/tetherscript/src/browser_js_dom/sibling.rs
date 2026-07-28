use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    for (name, after) in [("before", false), ("after", true)] {
        let h = handle.clone();
        obj.insert(
            name.into(),
            native(name, None, move |args| relative(&h, args, after)),
        );
    }
    let h = handle.clone();
    obj.insert(
        "replaceWith".into(),
        native("replaceWith", None, move |args| replace(&h, args)),
    );
}

fn relative(
    handle: &handle_ref::HandleRef,
    args: &[JsValue],
    after: bool,
) -> Result<JsValue, String> {
    let handle = handle.current();
    let Some((&index, parent_path)) = handle.path.split_last() else {
        return Ok(JsValue::Undefined);
    };
    let parent = DomHandle {
        root: handle.root.clone(),
        path: parent_path.to_vec(),
    };
    let index = if after {
        index.saturating_add(1)
    } else {
        index
    };
    insert_at::insert(&parent, index, node_args::from_values(args))?;
    Ok(JsValue::Undefined)
}

fn replace(handle: &handle_ref::HandleRef, args: &[JsValue]) -> Result<JsValue, String> {
    let handle = handle.current();
    let Some((&index, parent_path)) = handle.path.split_last() else {
        return Ok(JsValue::Undefined);
    };
    let parent = DomHandle {
        root: handle.root.clone(),
        path: parent_path.to_vec(),
    };
    replace_at::replace(&parent, index, node_args::from_values(args))?;
    Ok(JsValue::Undefined)
}
