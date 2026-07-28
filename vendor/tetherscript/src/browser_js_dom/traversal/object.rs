use super::*;

pub(super) fn traversal_object(
    root: DomHandle,
    paths: Vec<Vec<usize>>,
    kind: TraversalKind,
) -> JsValue {
    let state = Rc::new(RefCell::new(TraversalState::new(root.clone(), paths, kind)));
    let object_ref = Rc::new(RefCell::new(None));
    let mut obj = HashMap::new();
    obj.insert("root".into(), node_object(root.clone()));
    obj.insert("currentNode".into(), node_object(root));
    install_step(
        &mut obj,
        "nextNode",
        true,
        state.clone(),
        object_ref.clone(),
    );
    install_step(&mut obj, "previousNode", false, state, object_ref.clone());
    let object = Rc::new(RefCell::new(obj));
    *object_ref.borrow_mut() = Some(object.clone());
    JsValue::Object(object)
}
