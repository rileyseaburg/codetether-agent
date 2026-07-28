use super::*;

pub(super) fn src(handle: &DomHandle) -> String {
    attr(handle, "src").unwrap_or_default()
}

pub(super) fn duration(handle: &DomHandle) -> Option<f64> {
    attr(handle, "duration")
        .or_else(|| attr(handle, "data-duration"))
        .and_then(|value| value.parse::<f64>().ok())
}

pub(super) fn set(handle: &DomHandle, name: &str, value: String) {
    let name = name.to_ascii_lowercase();
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            el.attrs.insert(name, value);
        }
    });
}

fn attr(handle: &DomHandle, name: &str) -> Option<String> {
    match handle.node() {
        Some(Node::Element(el)) => el.attrs.get(name).cloned(),
        _ => None,
    }
}
