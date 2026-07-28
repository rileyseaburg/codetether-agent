use super::*;

pub(super) fn from_entry(entry: ShadowEntry) -> JsValue {
    let value = node_object(DomHandle {
        root: entry.root.clone(),
        path: Vec::new(),
    });
    if let JsValue::Object(obj) = &value {
        insert_metadata(obj, &entry);
    }
    value
}

fn insert_metadata(obj: &Rc<RefCell<HashMap<String, JsValue>>>, entry: &ShadowEntry) {
    let mut obj = obj.borrow_mut();
    obj.insert("mode".into(), JsValue::String(entry.mode.clone()));
    obj.insert("host".into(), host_metadata_object(&entry.host));
    obj.insert(
        "delegatesFocus".into(),
        JsValue::Bool(entry.delegates_focus),
    );
    obj.insert(
        "__shadowHostKey".into(),
        JsValue::String(entry.host.event_key()),
    );
}

fn host_metadata_object(host: &DomHandle) -> JsValue {
    let mut obj = HashMap::new();
    if let Some(Node::Element(el)) = host.node() {
        obj.insert("id".into(), attr(&el, "id"));
        obj.insert("tagName".into(), JsValue::String(el.tag.to_uppercase()));
        obj.insert("nodeName".into(), JsValue::String(el.tag.to_uppercase()));
    }
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn attr(element: &Element, name: &str) -> JsValue {
    JsValue::String(element.attrs.get(name).cloned().unwrap_or_default())
}
