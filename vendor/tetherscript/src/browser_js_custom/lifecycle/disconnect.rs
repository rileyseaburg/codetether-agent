use super::super::*;
use super::calls;

pub(super) fn subtree(node: Node) -> Result<(), String> {
    if let Node::Element(el) = node.clone() {
        call_current(node, &el)?;
        for child in el.children {
            subtree(child)?;
        }
    }
    Ok(())
}

fn call_current(node: Node, element: &Element) -> Result<(), String> {
    let Some(definition) = registry::get(&element.tag) else {
        return Ok(());
    };
    calls::call(
        &definition.value,
        "disconnectedCallback",
        detached_node_object(node),
        &[],
    )
}
