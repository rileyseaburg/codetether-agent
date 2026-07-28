use super::*;

mod methods;
mod props;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    let Node::Element(el) = node else {
        return;
    };
    if el.tag != "dialog" {
        return;
    }
    props::install(obj, handle, el);
    methods::install(obj, handle);
}
