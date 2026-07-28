use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    let object_ref = object.clone();
    let state_ref = state.clone();
    object.borrow_mut().insert(
        "collapse".into(),
        native("Range.collapse", None, move |args| {
            let to_start = args.first().map(JsValue::truthy).unwrap_or(true);
            let boundary = if to_start {
                state_ref.borrow().start.clone()
            } else {
                state_ref.borrow().end.clone()
            };
            let collapsed = state::RangeState::collapsed(&boundary.handle, boundary.offset);
            *state_ref.borrow_mut() = collapsed;
            props::refresh_range(&object_ref, &state_ref);
            Ok(JsValue::Undefined)
        }),
    );
}
