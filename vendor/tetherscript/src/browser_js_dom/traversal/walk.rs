use super::*;

pub(super) fn collect_paths(
    root: &DomHandle,
    mask: u32,
    filter: &JsValue,
    kind: TraversalKind,
) -> Result<Vec<Vec<usize>>, String> {
    let mut paths = Vec::new();
    visit(root, mask, filter, kind, &mut paths)?;
    Ok(paths)
}

pub(super) fn visit(
    handle: &DomHandle,
    mask: u32,
    filter: &JsValue,
    kind: TraversalKind,
    paths: &mut Vec<Vec<usize>>,
) -> Result<(), String> {
    let Some(node) = handle.node() else {
        return Ok(());
    };
    let mut descend = true;
    if is_shown(&node, mask) {
        match filter_result(filter, node_object(handle.clone()))? {
            FILTER_ACCEPT => paths.push(handle.path.clone()),
            FILTER_REJECT if matches!(kind, TraversalKind::TreeWalker) => descend = false,
            _ => {}
        }
    }
    if descend {
        visit_children(handle, &node, mask, filter, kind, paths)?;
    }
    Ok(())
}
