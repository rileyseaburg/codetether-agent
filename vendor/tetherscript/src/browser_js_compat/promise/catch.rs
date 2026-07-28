use super::*;

pub(super) fn method(
    state: Rc<RefCell<state::PromiseState>>,
    reactions: reaction::Queue,
) -> JsValue {
    native("Promise.catch", None, move |args| {
        let handler_value = args.first().cloned().unwrap_or(JsValue::Undefined);
        match state.borrow().clone() {
            state::PromiseState::Rejected(reason) => Ok(reaction::settled_then(
                JsValue::Undefined,
                handler_value,
                state::PromiseState::Rejected(reason),
            )),
            state::PromiseState::Fulfilled(value) => Ok(reaction::settled_then(
                JsValue::Undefined,
                handler_value,
                state::PromiseState::Fulfilled(value),
            )),
            state::PromiseState::Pending => Ok(reaction::push_then(
                reactions.clone(),
                JsValue::Undefined,
                handler_value,
            )),
        }
    })
}
