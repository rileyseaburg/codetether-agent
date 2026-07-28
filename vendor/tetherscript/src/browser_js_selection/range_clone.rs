use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &Rc<RefCell<state::RangeState>>,
) {
    let state_ref = state.clone();
    object.borrow_mut().insert(
        "cloneRange".into(),
        native("Range.cloneRange", Some(0), move |_| {
            Ok(range_object::object(state_ref.borrow().clone()))
        }),
    );
}
