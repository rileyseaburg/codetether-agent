use super::*;

pub(super) fn value(handle: &DomHandle, name: &str) -> Option<String> {
    match handle.node() {
        Some(Node::Element(el)) => el.attrs.get(name).cloned(),
        _ => None,
    }
}

pub(super) fn set(handle: &DomHandle, name: &str, value: String) -> Result<(), String> {
    change(handle, name, Some(value))
}

pub(super) fn remove(handle: &DomHandle, name: &str) -> Result<(), String> {
    change(handle, name, None)
}

pub(super) fn change_style(handle: &DomHandle, value: Option<String>) -> Result<(), String> {
    change(handle, "style", value)
}

fn change(handle: &DomHandle, name: &str, next: Option<String>) -> Result<(), String> {
    let old_value = value(handle, name);
    let next_value = next.clone();
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            match next {
                Some(value) => {
                    el.attrs.insert(name.into(), value);
                }
                None => {
                    el.attrs.remove(name);
                }
            }
        }
    });
    queue_mutation_record(
        handle,
        "attributes",
        Some(name.into()),
        old_value.clone(),
        Vec::new(),
        Vec::new(),
    );
    custom_host::attribute_changed(handle, name, old_value, next_value)
}
