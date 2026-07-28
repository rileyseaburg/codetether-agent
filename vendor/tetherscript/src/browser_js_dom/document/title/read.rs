use super::*;

pub(super) fn read(handle: &DomHandle) -> String {
    let root = handle.root.borrow();
    find_tag(&root, "title", &mut Vec::new())
        .and_then(|path| get_node(&root, &path).map(text_content_raw))
        .unwrap_or_default()
}
