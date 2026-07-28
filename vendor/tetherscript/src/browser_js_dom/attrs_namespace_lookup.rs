use super::super::super::*;
use super::attrs_namespace_meta;

const XLINK_NS: &str = "http://www.w3.org/1999/xlink";

pub(super) fn qualified(
    handle: &DomHandle,
    namespace: &Option<String>,
    local: &str,
) -> Option<String> {
    attrs_namespace_meta::find(handle, namespace, local)
        .filter(|name| exists(handle, name))
        .or_else(|| inferred(handle, namespace, local))
}

fn inferred(handle: &DomHandle, namespace: &Option<String>, local: &str) -> Option<String> {
    let name = match namespace.as_deref() {
        None => local.to_string(),
        Some(XLINK_NS) => format!("xlink:{local}"),
        Some(_) => return None,
    };
    match handle.node() {
        Some(Node::Element(el)) if el.attrs.contains_key(&name) => Some(name),
        _ => None,
    }
}

fn exists(handle: &DomHandle, name: &str) -> bool {
    matches!(handle.node(), Some(Node::Element(el)) if el.attrs.contains_key(name))
}
