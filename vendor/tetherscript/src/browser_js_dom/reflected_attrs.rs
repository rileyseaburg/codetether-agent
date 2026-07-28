use super::*;

#[path = "reflected_disabled.rs"]
mod reflected_disabled;

const STRING_ATTRS: [&str; 7] = ["name", "type", "href", "src", "alt", "title", "placeholder"];

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    let Node::Element(el) = node else {
        return;
    };
    if el.tag.starts_with('#') {
        return;
    }
    for attr in STRING_ATTRS {
        install_string_attr(obj, handle, el, attr);
    }
    reflected_disabled::install(obj, handle, el);
}

fn install_string_attr(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    el: &Element,
    attr: &'static str,
) {
    obj.entry(attr.into())
        .or_insert_with(|| JsValue::String(el.attrs.get(attr).cloned().unwrap_or_default()));
    let key = format!("__set:{attr}");
    if obj.contains_key(&key) {
        return;
    }
    let h = handle_ref::new(obj, handle);
    obj.insert(
        key,
        native("set_reflected_attr", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            attr_update::set(&h.current(), attr, value.clone())?;
            Ok(JsValue::String(value))
        }),
    );
}
