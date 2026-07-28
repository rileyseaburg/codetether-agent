use super::*;

pub(super) fn insert(parent: &DomHandle, index: usize, nodes: Vec<Node>) -> Result<(), String> {
    if nodes.is_empty() {
        return Ok(());
    }
    let added = nodes.clone();
    let count = nodes.len();
    let start = {
        let mut root = parent.root.borrow_mut();
        let Some(Node::Element(el)) = get_node_mut(&mut root, &parent.path) else {
            return Ok(());
        };
        let start = index.min(el.children.len());
        for (offset, node) in nodes.into_iter().enumerate() {
            el.children.insert(start + offset, node);
        }
        start
    };
    path_shift::inserted(parent, start, count);
    queue_mutation_record(parent, "childList", None, None, added, Vec::new());
    inserted::connect(parent, start, count)
}
