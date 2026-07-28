use super::*;

pub(super) fn install_document(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let root = handle.clone();
    obj.insert(
        "createRange".into(),
        native("document.createRange", Some(0), move |_| {
            Ok(object(state::RangeState::collapsed(&root, 0)))
        }),
    );
}

pub(super) fn object(initial: state::RangeState) -> JsValue {
    let state = Rc::new(RefCell::new(initial));
    let object = Rc::new(RefCell::new(HashMap::new()));
    let id = registry::register_range(state.clone());
    object
        .borrow_mut()
        .insert("__rangeId".into(), JsValue::String(id));
    props::refresh_range(&object, &state);
    range_select::install(&object, &state);
    range_offsets::install(&object, &state);
    range_collapse::install(&object, &state);
    range_clone::install(&object, &state);
    install_text(&object, &state);
    JsValue::Object(object)
}

fn install_text(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    let text_state = state.clone();
    object.borrow_mut().insert(
        "toString".into(),
        native("Range.toString", Some(0), move |_| {
            Ok(JsValue::String(text::range_text(&text_state.borrow())))
        }),
    );
}
