use super::*;

pub(super) const FILTER_ACCEPT: i32 = 1;
pub(super) const FILTER_REJECT: i32 = 2;
pub(super) const FILTER_SKIP: i32 = 3;
pub(super) const SHOW_ALL: u32 = u32::MAX;
pub(super) const SHOW_ELEMENT: u32 = 0x1;
pub(super) const SHOW_TEXT: u32 = 0x4;

pub(super) fn node_filter_object() -> JsValue {
    let mut obj = HashMap::new();
    for (name, value) in [
        ("SHOW_ALL", SHOW_ALL as f64),
        ("SHOW_ELEMENT", SHOW_ELEMENT as f64),
        ("SHOW_TEXT", SHOW_TEXT as f64),
        ("FILTER_ACCEPT", FILTER_ACCEPT as f64),
        ("FILTER_REJECT", FILTER_REJECT as f64),
        ("FILTER_SKIP", FILTER_SKIP as f64),
    ] {
        obj.insert(name.into(), JsValue::Number(value));
    }
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

pub(super) fn show_mask(value: Option<&JsValue>) -> u32 {
    match value {
        Some(JsValue::Number(n)) if n.is_finite() && *n >= 0.0 => *n as u32,
        Some(value) => value.display().parse().unwrap_or(SHOW_ALL),
        None => SHOW_ALL,
    }
}

pub(super) fn is_shown(node: &Node, mask: u32) -> bool {
    mask == SHOW_ALL || node_mask(node).is_some_and(|bit| mask & bit != 0)
}

fn node_mask(node: &Node) -> Option<u32> {
    match node {
        Node::Element(el) if el.tag == "#document" => Some(0x100),
        Node::Element(el) if el.tag == "#document-fragment" => Some(0x400),
        Node::Element(_) => Some(SHOW_ELEMENT),
        Node::Text(_) => Some(SHOW_TEXT),
    }
}
