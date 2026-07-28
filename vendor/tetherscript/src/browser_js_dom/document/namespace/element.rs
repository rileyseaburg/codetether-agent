use super::super::super::*;
use super::name;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "createElementNS".into(),
        native("document.createElementNS", Some(2), |args| Ok(create(args))),
    );
}

fn create(args: &[JsValue]) -> JsValue {
    let namespace = name::namespace_value(args.first());
    let qualified = name::parse(args.get(1).unwrap_or(&JsValue::Undefined));
    let element = ops::detached_object(Node::Element(Element {
        tag: qualified.name.clone(),
        attrs: HashMap::new(),
        children: Vec::new(),
    }));
    apply_namespace_fields(&element, namespace, qualified);
    element
}

fn apply_namespace_fields(element: &JsValue, namespace: JsValue, qualified: name::QualifiedName) {
    let JsValue::Object(obj) = element else {
        return;
    };
    let node_name = name::node_name(&namespace, &qualified.name);
    let prefix = name::prefix_value(&qualified);
    let mut obj = obj.borrow_mut();
    obj.insert("namespaceURI".into(), namespace);
    obj.insert("localName".into(), JsValue::String(qualified.local));
    obj.insert("prefix".into(), prefix);
    obj.insert("nodeName".into(), JsValue::String(node_name.clone()));
    obj.insert("tagName".into(), JsValue::String(node_name));
}
