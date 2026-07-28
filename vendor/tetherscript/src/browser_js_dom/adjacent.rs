use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle_ref::new(obj, handle);
    obj.insert(
        "insertAdjacentHTML".into(),
        native("insertAdjacentHTML", Some(2), move |args| {
            let position = args.first().unwrap_or(&JsValue::Undefined).display();
            let html = args.get(1).unwrap_or(&JsValue::Undefined).display();
            adjacent_position::insert(&h.current(), &position, node_args::from_html(&html))?;
            Ok(JsValue::Undefined)
        }),
    );
    let h = handle_ref::new(obj, handle);
    obj.insert(
        "insertAdjacentText".into(),
        native("insertAdjacentText", Some(2), move |args| {
            let position = args.first().unwrap_or(&JsValue::Undefined).display();
            let text = args.get(1).unwrap_or(&JsValue::Undefined).display();
            adjacent_position::insert(&h.current(), &position, node_args::from_text(text))?;
            Ok(JsValue::Undefined)
        }),
    );
    let h = handle_ref::new(obj, handle);
    obj.insert(
        "insertAdjacentElement".into(),
        native("insertAdjacentElement", Some(2), move |args| {
            let position = args.first().unwrap_or(&JsValue::Undefined).display();
            let element = args.get(1).unwrap_or(&JsValue::Undefined);
            let Some(node) = element_node(element) else {
                return Ok(JsValue::Null);
            };
            adjacent_position::insert(&h.current(), &position, vec![node])?;
            Ok(element.clone())
        }),
    );
}

fn element_node(value: &JsValue) -> Option<Node> {
    let node = dom_handle_from_value(value)?.node()?;
    match node {
        Node::Element(el) if !el.tag.starts_with('#') => Some(Node::Element(el)),
        _ => None,
    }
}
