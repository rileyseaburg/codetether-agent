use super::super::*;

#[path = "scroll_metrics_edges.rs"]
mod scroll_metrics_edges;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    if !real_element(node) {
        return;
    }
    install_metrics(obj, handle);
    install_methods(obj);
}

fn real_element(node: &Node) -> bool {
    matches!(node, Node::Element(el) if !el.tag.starts_with('#'))
}

fn install_metrics(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let (x, y, width, height) = element_rect(handle);
    let [top, right, bottom, left] = scroll_metrics_edges::border(handle);
    let client_width = (width - left - right).max(0);
    let client_height = (height - top - bottom).max(0);
    for (name, value) in [
        ("clientWidth", client_width),
        ("clientHeight", client_height),
        ("scrollWidth", client_width),
        ("scrollHeight", client_height),
        ("offsetLeft", x),
        ("offsetTop", y),
        ("clientLeft", left),
        ("clientTop", top),
    ] {
        obj.insert(name.into(), JsValue::Number(value as f64));
    }
}

fn install_methods(obj: &mut HashMap<String, JsValue>) {
    for name in ["scrollIntoView", "scrollTo", "scrollBy"] {
        obj.insert(name.into(), native(name, None, |_| Ok(JsValue::Undefined)));
    }
}

#[cfg(test)]
#[path = "scroll_metrics_tests.rs"]
mod scroll_metrics_tests;
