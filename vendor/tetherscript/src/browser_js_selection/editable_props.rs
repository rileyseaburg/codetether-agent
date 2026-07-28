use super::*;

type ObjectCell = Rc<RefCell<HashMap<String, JsValue>>>;
type NodeObjectRef = Rc<RefCell<Option<ObjectCell>>>;

pub(in crate::browser_js) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    element: &Element,
    owner: NodeObjectRef,
) {
    refresh(obj, handle, element);
    if obj.contains_key("__set:contentEditable") {
        return;
    }
    let h = handle.clone();
    obj.insert(
        "__set:contentEditable".into(),
        native("set_contentEditable", Some(1), move |args| {
            let raw = args.first().unwrap_or(&JsValue::Undefined).display();
            let next = editable_state::set_value(&raw);
            editable_attr::set(&h, next.clone())?;
            if let Some(object) = owner.borrow().as_ref() {
                refresh_object(object, &h);
            }
            Ok(JsValue::String(next.unwrap_or_else(|| "inherit".into())))
        }),
    );
}

fn refresh_object(object: &ObjectCell, handle: &DomHandle) {
    let Some(Node::Element(element)) = handle.node() else {
        return;
    };
    refresh(&mut object.borrow_mut(), handle, &element);
}

fn refresh(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, element: &Element) {
    obj.insert(
        "contentEditable".into(),
        JsValue::String(editable_state::prop(element)),
    );
    obj.insert(
        "isContentEditable".into(),
        JsValue::Bool(editable_state::effective(handle)),
    );
}
