use super::*;

pub(super) fn replace(parent: &DomHandle, index: usize, nodes: Vec<Node>) -> Result<(), String> {
    let added = nodes.clone();
    let count = nodes.len();
    let removed = {
        let mut root = parent.root.borrow_mut();
        let Some(Node::Element(el)) = get_node_mut(&mut root, &parent.path) else {
            return Ok(());
        };
        if index >= el.children.len() {
            return Ok(());
        }
        let removed = el.children.remove(index);
        for (offset, node) in nodes.into_iter().enumerate() {
            el.children.insert(index + offset, node);
        }
        removed
    };
    path_shift::replaced(parent, index, count);
    queue_mutation_record(
        parent,
        "childList",
        None,
        None,
        added,
        vec![removed.clone()],
    );
    custom_host::disconnected(removed)?;
    inserted::connect(parent, index, count)
}
