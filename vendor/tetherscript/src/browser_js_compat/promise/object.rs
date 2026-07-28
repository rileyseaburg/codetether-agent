use super::*;

#[path = "object/methods.rs"]
mod methods;

pub(super) fn from_state(state: state::PromiseState) -> JsValue {
    let shared = Rc::new(RefCell::new(state));
    JsValue::Object(from_parts(shared, reaction::queue()))
}

pub(super) fn from_parts(
    state: Rc<RefCell<state::PromiseState>>,
    reactions: reaction::Queue,
) -> Rc<RefCell<HashMap<String, JsValue>>> {
    let mut object = HashMap::new();
    write_state(&mut object, &state.borrow());
    methods::install(&mut object, state, reactions);
    Rc::new(RefCell::new(object))
}

pub(super) fn write_state(object: &mut HashMap<String, JsValue>, state: &state::PromiseState) {
    object.remove("__promise_value");
    object.remove("__promise_reason");
    object.remove("value");
    object.remove("reason");
    match state {
        state::PromiseState::Pending => {
            object.insert("__promise_state".into(), JsValue::String("pending".into()));
        }
        state::PromiseState::Fulfilled(value) => {
            object.insert(
                "__promise_state".into(),
                JsValue::String("fulfilled".into()),
            );
            object.insert("__promise_value".into(), value.clone());
            object.insert("value".into(), value.clone());
        }
        state::PromiseState::Rejected(reason) => {
            object.insert("__promise_state".into(), JsValue::String("rejected".into()));
            object.insert("__promise_reason".into(), reason.clone());
            object.insert("reason".into(), reason.clone());
        }
    }
}
