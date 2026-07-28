use super::*;

pub(super) fn install(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    node: &Node,
) {
    let Some(el) = controls::element(node) else {
        return;
    };
    match el.tag.as_str() {
        "form" => install_form(obj, handle),
        "option" => select::install_option(obj, handle),
        _ if controls::is_control(el) => install_control(obj, handle, el),
        _ => {}
    }
}

fn install_form(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    select::sync_tree(handle);
    form_props::install(obj, handle);
    form_methods::install(obj, handle);
}

fn install_control(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle, el: &Element) {
    refresh::write(obj, handle);
    control_methods::install(obj, handle);
    if el.tag == "select" {
        select::install(obj, handle);
    } else {
        value_setter::install(obj, handle);
        input_compat::install(obj, handle);
    }
}
