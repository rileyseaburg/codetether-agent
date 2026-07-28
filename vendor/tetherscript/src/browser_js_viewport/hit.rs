use super::*;

pub(super) fn single_at(
    root: &Rc<RefCell<Node>>,
    viewport_x: i64,
    viewport_y: i64,
    document_x: i64,
    document_y: i64,
) -> JsValue {
    if !inside_viewport(viewport_x, viewport_y) {
        return JsValue::Null;
    }
    hit_collect::hits(root, document_x, document_y)
        .into_iter()
        .next()
        .map(|hit| node_for(root, hit.path))
        .unwrap_or(JsValue::Null)
}

pub(super) fn all_at(
    root: &Rc<RefCell<Node>>,
    viewport_x: i64,
    viewport_y: i64,
    document_x: i64,
    document_y: i64,
) -> JsValue {
    if !inside_viewport(viewport_x, viewport_y) {
        return JsValue::Array(Rc::new(RefCell::new(Vec::new())));
    }
    let nodes = hit_collect::hits(root, document_x, document_y)
        .into_iter()
        .map(|hit| node_for(root, hit.path))
        .collect();
    JsValue::Array(Rc::new(RefCell::new(nodes)))
}

fn node_for(root: &Rc<RefCell<Node>>, path: Vec<usize>) -> JsValue {
    node_object(DomHandle {
        root: root.clone(),
        path,
    })
}

fn inside_viewport(x: i64, y: i64) -> bool {
    x >= 0
        && y >= 0
        && x < constants::DEFAULT_VIEWPORT_WIDTH
        && y < constants::DEFAULT_VIEWPORT_HEIGHT
}
