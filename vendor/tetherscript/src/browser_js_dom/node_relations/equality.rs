use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "isEqualNode".into(),
        native("Node.isEqualNode", Some(1), move |args| {
            let Some(other) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Bool(false));
            };
            Ok(JsValue::Bool(equal_node(&handle.current(), &other)))
        }),
    );
}

fn equal_node(left: &DomHandle, right: &DomHandle) -> bool {
    match (left.node(), right.node()) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}
