use super::*;

pub(super) fn insert(handle: &DomHandle, position: &str, nodes: Vec<Node>) -> Result<(), String> {
    match position.to_ascii_lowercase().as_str() {
        "afterbegin" => insert_at::insert(handle, 0, nodes),
        "beforeend" => insert_at::insert(handle, usize::MAX, nodes),
        "beforebegin" => parent_insert(handle, 0, nodes),
        "afterend" => parent_insert(handle, 1, nodes),
        _ => Err(format!("insertAdjacent: invalid position {}", position)),
    }
}

fn parent_insert(handle: &DomHandle, offset: usize, nodes: Vec<Node>) -> Result<(), String> {
    let Some((&index, parent_path)) = handle.path.split_last() else {
        return Ok(());
    };
    let parent = DomHandle {
        root: handle.root.clone(),
        path: parent_path.to_vec(),
    };
    insert_at::insert(&parent, index.saturating_add(offset), nodes)
}
