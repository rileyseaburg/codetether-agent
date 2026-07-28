use super::*;

#[path = "document/anchors.rs"]
mod anchors;
#[path = "document/collections.rs"]
mod collections;
#[path = "document/events.rs"]
mod events;
#[path = "document/implementation.rs"]
mod implementation;
#[path = "document/import.rs"]
mod import;
#[path = "document/legacy.rs"]
mod legacy;
#[path = "document/lookup.rs"]
mod lookup;
#[path = "document/metadata.rs"]
mod metadata;
#[path = "document/namespace.rs"]
mod namespace;
#[path = "document/title.rs"]
mod title;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    if !matches!(node, Node::Element(el) if el.tag == "#document") {
        return;
    }
    let document_element = lookup::element_or_first(handle, "html");
    obj.insert("documentElement".into(), document_element.clone());
    obj.insert("scrollingElement".into(), document_element);
    obj.insert("head".into(), lookup::element_or_new(handle, "head"));
    obj.insert("body".into(), lookup::element_or_first(handle, "body"));
    anchors::install(obj, handle);
    collections::install(obj, handle);
    metadata::install(obj);
    events::install(obj);
    legacy::install(obj, handle);
    title::install(obj, handle);
    import::install(obj);
    implementation::install(obj);
    namespace::install(obj);
}
