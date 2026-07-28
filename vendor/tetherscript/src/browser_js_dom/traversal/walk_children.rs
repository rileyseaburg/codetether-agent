use super::walk::visit;
use super::*;

pub(super) fn visit_children(
    handle: &DomHandle,
    node: &Node,
    mask: u32,
    filter: &JsValue,
    kind: TraversalKind,
    paths: &mut Vec<Vec<usize>>,
) -> Result<(), String> {
    let Node::Element(el) = node else {
        return Ok(());
    };
    for index in 0..el.children.len() {
        visit(&child_handle(handle, index), mask, filter, kind, paths)?;
    }
    Ok(())
}

fn child_handle(handle: &DomHandle, index: usize) -> DomHandle {
    let mut path = handle.path.clone();
    path.push(index);
    DomHandle {
        root: handle.root.clone(),
        path,
    }
}
