use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    if !matches!(node, Node::Element(el) if !el.tag.starts_with('#')) {
        return;
    }
    let h = handle.clone();
    obj.insert(
        "getClientRects".into(),
        native("getClientRects", Some(0), move |_| Ok(list(&h))),
    );
}

fn list(handle: &DomHandle) -> JsValue {
    let rect = element_rect(handle);
    let values = if visible(rect) {
        vec![rect_object(&rect)]
    } else {
        Vec::new()
    };
    dom_collection("DOMRectList", values)
}

fn visible((_, _, width, height): (i64, i64, i64, i64)) -> bool {
    width > 0 && height > 0
}

#[cfg(test)]
#[path = "tests_client_rects.rs"]
mod tests_client_rects;
#[cfg(test)]
#[path = "tests_dom_rect.rs"]
mod tests_dom_rect;
