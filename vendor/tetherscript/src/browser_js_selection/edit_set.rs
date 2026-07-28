use super::*;

pub(super) fn content(handle: &DomHandle, value: String) {
    let old_value = handle.node().map(|node| text_content_raw(&node));
    handle.with_node_mut(|node| {
        if let Node::Element(element) = node {
            element.children = vec![Node::Text(value)];
        }
    });
    queue_mutation_record(
        handle,
        "characterData",
        None,
        old_value,
        Vec::new(),
        Vec::new(),
    );
}
