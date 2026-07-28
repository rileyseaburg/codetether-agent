use super::super::*;

#[path = "title/read.rs"]
mod read;
#[path = "title/write.rs"]
mod write;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    obj.insert("title".into(), JsValue::String(read::read(handle)));
    let h = handle.clone();
    obj.insert(
        "__set:title".into(),
        native("set_document_title", Some(1), move |args| {
            let text = args.first().unwrap_or(&JsValue::Undefined).display();
            write::write(&h, text.clone());
            Ok(JsValue::String(text))
        }),
    );
}

pub(super) fn find_tag(node: &Node, tag: &str, path: &mut Vec<usize>) -> Option<Vec<usize>> {
    if matches!(node, Node::Element(el) if el.tag == tag) {
        return Some(path.clone());
    }
    let Node::Element(el) = node else { return None };
    for (index, child) in el.children.iter().enumerate() {
        path.push(index);
        if let Some(found) = find_tag(child, tag, path) {
            return Some(found);
        }
        path.pop();
    }
    None
}

pub(super) fn element(tag: &str, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        attrs: HashMap::new(),
        children,
    })
}
