use super::*;

thread_local! {
    static ROOTS: RefCell<HashMap<String, ShadowEntry>> = RefCell::new(HashMap::new());
}

pub(super) fn reset() {
    ROOTS.with(|roots| roots.borrow_mut().clear());
}

pub(super) fn get(host: &DomHandle) -> Option<ShadowEntry> {
    ROOTS.with(|roots| roots.borrow().get(&host.event_key()).cloned())
}

pub(super) fn entry(host: &DomHandle, mode: String, delegates_focus: bool) -> ShadowEntry {
    ROOTS.with(|roots| {
        roots
            .borrow_mut()
            .entry(host.event_key())
            .or_insert_with(|| ShadowEntry {
                root: fragment_root(),
                host: host.clone(),
                mode,
                delegates_focus,
            })
            .clone()
    })
}

fn fragment_root() -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node::Element(Element {
        tag: "#document-fragment".into(),
        attrs: HashMap::new(),
        children: Vec::new(),
    })))
}
