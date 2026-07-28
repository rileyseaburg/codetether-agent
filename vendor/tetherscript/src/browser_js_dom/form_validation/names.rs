use super::*;

pub(super) fn for_handle(handle: &DomHandle) -> Vec<String> {
    let Some(Node::Element(el)) = handle.node() else {
        return Vec::new();
    };
    ["id", "name"]
        .into_iter()
        .filter_map(|attr| el.attrs.get(attr))
        .filter(|value| !value.is_empty())
        .cloned()
        .collect()
}

pub(super) fn matches(handle: &DomHandle, name: &str) -> bool {
    let Some(Node::Element(el)) = handle.node() else {
        return false;
    };
    ["id", "name"]
        .into_iter()
        .filter_map(|attr| el.attrs.get(attr))
        .any(|value| value == name)
}
