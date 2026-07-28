use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    let select_object = object.clone();
    let select_state = state.clone();
    object.borrow_mut().insert(
        "selectNode".into(),
        native("Range.selectNode", Some(1), move |args| {
            let Some(handle) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Undefined);
            };
            *select_state.borrow_mut() = state::RangeState::select_node(&handle);
            props::refresh_range(&select_object, &select_state);
            Ok(JsValue::Undefined)
        }),
    );

    let contents_object = object.clone();
    let contents_state = state.clone();
    object.borrow_mut().insert(
        "selectNodeContents".into(),
        native("Range.selectNodeContents", Some(1), move |args| {
            let Some(handle) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Undefined);
            };
            *contents_state.borrow_mut() = state::RangeState::select_contents(&handle);
            props::refresh_range(&contents_object, &contents_state);
            Ok(JsValue::Undefined)
        }),
    );
}
