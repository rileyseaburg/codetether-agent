use super::*;

pub(super) fn root(markup: &str) -> Rc<RefCell<Node>> {
    let doc = browser::parse_html(markup);
    Rc::new(RefCell::new(Node::Element(Element {
        tag: "#document".into(),
        attrs: HashMap::new(),
        children: doc.children,
    })))
}
