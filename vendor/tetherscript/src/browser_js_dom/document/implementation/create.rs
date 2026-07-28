use super::super::super::*;

pub(super) fn document(args: &[JsValue]) -> JsValue {
    let title = args.first().unwrap_or(&JsValue::Undefined).display();
    ops::document_object(root(title))
}

fn root(title: String) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(element(
        "#document",
        vec![element(
            "html",
            vec![head(title), element("body", Vec::new())],
        )],
    )))
}

fn head(title: String) -> Node {
    element("head", vec![element("title", vec![Node::Text(title)])])
}

fn element(tag: &str, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        attrs: HashMap::new(),
        children,
    })
}
