use super::*;

#[path = "lifecycle/attributes.rs"]
mod attributes;
#[path = "lifecycle/calls.rs"]
mod calls;
#[path = "lifecycle/connect.rs"]
mod connect;
#[path = "lifecycle/disconnect.rs"]
mod disconnect;

pub(super) fn construct_created(tag: &str, element: &JsValue) -> Result<(), String> {
    let Some(definition) = registry::get(tag) else {
        return Ok(());
    };
    calls::call(&definition.value, "constructor", element.clone(), &[])
}

pub(super) fn connected(handle: &DomHandle) -> Result<(), String> {
    if !matches!(&*handle.root.borrow(), Node::Element(el) if el.tag == "#document") {
        return Ok(());
    }
    connect::subtree(handle)
}

pub(super) fn disconnected(node: Node) -> Result<(), String> {
    disconnect::subtree(node)
}

pub(super) fn attribute_changed(
    handle: &DomHandle,
    name: &str,
    old_value: Option<String>,
    new_value: Option<String>,
) -> Result<(), String> {
    attributes::changed(handle, name, old_value, new_value)
}
