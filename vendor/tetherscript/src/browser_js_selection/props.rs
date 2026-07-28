use super::*;

pub(super) fn refresh_range(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    let state = state.borrow();
    let mut object = object.borrow_mut();
    object.insert(
        "startContainer".into(),
        node_object(state.start.handle.clone()),
    );
    object.insert(
        "startOffset".into(),
        JsValue::Number(state.start.offset as f64),
    );
    object.insert("endContainer".into(), node_object(state.end.handle.clone()));
    object.insert("endOffset".into(), JsValue::Number(state.end.offset as f64));
    object.insert("collapsed".into(), JsValue::Bool(state.is_collapsed()));
    object.insert(
        "commonAncestorContainer".into(),
        node_object(common::ancestor(&state)),
    );
}

pub(super) fn refresh_selection(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let selected = registry::selection();
    let collapsed = selected
        .as_ref()
        .map(state::RangeState::is_collapsed)
        .unwrap_or_else(|| selection_text().is_empty());
    let mut object = object.borrow_mut();
    object.insert(
        "rangeCount".into(),
        JsValue::Number(selected.is_some() as u8 as f64),
    );
    object.insert("isCollapsed".into(), JsValue::Bool(collapsed));
    object.insert("collapsed".into(), JsValue::Bool(collapsed));
}

pub(super) fn selection_text() -> String {
    registry::selection()
        .map(|state| text::range_text(&state))
        .or_else(control::active_control_text)
        .unwrap_or_default()
}
