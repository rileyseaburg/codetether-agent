use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, node: &Node) {
    let Node::Element(el) = node else {
        return;
    };
    if el.tag != "template" {
        return;
    }
    obj.insert(
        "content".into(),
        construct::fragment_from_children(el.children.clone()),
    );
}
