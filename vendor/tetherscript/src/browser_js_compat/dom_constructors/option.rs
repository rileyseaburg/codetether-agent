use super::*;

pub(super) fn create(args: &[JsValue]) -> Result<JsValue, String> {
    let text = string_arg(args, 0, "");
    let value = string_arg(args, 1, &text);
    let default_selected = bool_arg(args, 2, false);
    let selected = bool_arg(args, 3, default_selected);
    let mut attrs = HashMap::from([("value".into(), value)]);
    if default_selected || selected {
        attrs.insert("selected".into(), String::new());
    }
    let option = detached_node_object(Node::Element(Element {
        tag: "option".into(),
        attrs,
        children: vec![Node::Text(text)],
    }));
    decorate(&option, default_selected, selected);
    Ok(option)
}

fn decorate(option: &JsValue, default_selected: bool, selected: bool) {
    let Some(obj) = object(option) else {
        return;
    };
    let mut obj = obj.borrow_mut();
    obj.insert("defaultSelected".into(), JsValue::Bool(default_selected));
    obj.insert("selected".into(), JsValue::Bool(selected));
}
