use super::super::*;
use super::Object;

pub(super) fn reaction(
    ok: JsValue,
    err: JsValue,
    current: state::PromiseState,
    state: Rc<RefCell<state::PromiseState>>,
    object: Object,
    queue: reaction::Queue,
) {
    match super::action::from_handlers(ok, err, current) {
        super::action::Action::Ready(next) => reaction::settle(&state, &object, &queue, next),
        super::action::Action::Wait(value) => wait(value, state, object, queue),
    }
}

fn wait(
    value: JsValue,
    state: Rc<RefCell<state::PromiseState>>,
    object: Object,
    queue: reaction::Queue,
) {
    if let Err(error) = super::link::attach(&value, state.clone(), object.clone(), queue.clone()) {
        reaction::settle(
            &state,
            &object,
            &queue,
            state::PromiseState::Rejected(JsValue::String(error)),
        );
    }
}
