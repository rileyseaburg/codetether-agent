use super::{super::*, action::Action};

#[path = "callbacks.rs"]
mod callbacks;
#[path = "link.rs"]
mod link;

pub(super) fn settle_into(
    action: Action,
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    queue: reaction::Queue,
) {
    match action {
        Action::Ready(next) => reaction::settle(&state, &object, &queue, next),
        Action::Wait(value, original) => {
            if let Err(error) = link::attach(
                &value,
                original,
                state.clone(),
                object.clone(),
                queue.clone(),
            ) {
                reaction::settle(
                    &state,
                    &object,
                    &queue,
                    state::PromiseState::Rejected(JsValue::String(error)),
                );
            }
        }
    }
}
