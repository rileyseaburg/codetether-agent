use super::*;

pub(super) fn install_step(
    obj: &mut HashMap<String, JsValue>,
    name: &'static str,
    forward: bool,
    state: Rc<RefCell<TraversalState>>,
    object_ref: TraversalObjectRef,
) {
    obj.insert(
        name.into(),
        native(name, Some(0), move |_| {
            let handle = if forward {
                state.borrow_mut().next()
            } else {
                state.borrow_mut().previous()
            };
            Ok(match handle {
                Some(handle) => set_current(&object_ref, node_object(handle)),
                None => JsValue::Null,
            })
        }),
    );
}

fn set_current(object_ref: &TraversalObjectRef, node: JsValue) -> JsValue {
    if let Some(object) = object_ref.borrow().as_ref() {
        object
            .borrow_mut()
            .insert("currentNode".into(), node.clone());
    }
    node
}
