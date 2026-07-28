use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    install_boundary(object, state, true);
    install_boundary(object, state, false);
}

fn install_boundary(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
    start: bool,
) {
    let object_ref = object.clone();
    let state_ref = state.clone();
    let name = if start { "setStart" } else { "setEnd" };
    object.borrow_mut().insert(
        name.into(),
        native(&format!("Range.{name}"), Some(2), move |args| {
            let Some(handle) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Undefined);
            };
            let offset = args.get(1).map(selection_index).unwrap_or(0);
            let mut state = state_ref.borrow_mut();
            if start {
                state.start = state::boundary(&handle, offset);
            } else {
                state.end = state::boundary(&handle, offset);
            }
            drop(state);
            props::refresh_range(&object_ref, &state_ref);
            Ok(JsValue::Undefined)
        }),
    );
}
