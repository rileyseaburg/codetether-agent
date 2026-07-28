use super::super::*;
use super::calls;

pub(super) fn subtree(handle: &DomHandle) -> Result<(), String> {
    if let Some(Node::Element(el)) = handle.node() {
        call_current(handle, &el)?;
        for index in 0..el.children.len() {
            let mut path = handle.path.clone();
            path.push(index);
            subtree(&DomHandle {
                root: handle.root.clone(),
                path,
            })?;
        }
    }
    Ok(())
}

fn call_current(handle: &DomHandle, element: &Element) -> Result<(), String> {
    let Some(definition) = registry::get(&element.tag) else {
        return Ok(());
    };
    calls::call(
        &definition.value,
        "connectedCallback",
        node_object(handle.clone()),
        &[],
    )
}
