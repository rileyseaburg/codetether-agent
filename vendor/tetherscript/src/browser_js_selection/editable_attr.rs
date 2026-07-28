use super::*;

pub(super) fn set(handle: &DomHandle, next: Option<String>) -> Result<(), String> {
    let old_value = handle.node().and_then(|node| match node {
        Node::Element(el) => el.attrs.get("contenteditable").cloned(),
        Node::Text(_) => None,
    });
    let next_value = next.clone();
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            match next {
                Some(value) => {
                    el.attrs.insert("contenteditable".into(), value);
                }
                None => {
                    el.attrs.remove("contenteditable");
                }
            }
        }
    });
    queue_mutation_record(
        handle,
        "attributes",
        Some("contenteditable".into()),
        old_value.clone(),
        Vec::new(),
        Vec::new(),
    );
    custom_host::attribute_changed(handle, "contenteditable", old_value, next_value)
}
