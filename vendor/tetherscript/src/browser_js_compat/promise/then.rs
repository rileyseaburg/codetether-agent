use super::*;

#[path = "then/action.rs"]
mod action;
#[path = "then/link.rs"]
mod link;
#[path = "then/settle.rs"]
mod settle;

type Object = Rc<RefCell<HashMap<String, JsValue>>>;

pub(super) fn method(
    state: Rc<RefCell<state::PromiseState>>,
    reactions: reaction::Queue,
) -> JsValue {
    native("Promise.then", None, move |args| {
        let on_ok = args.first().cloned().unwrap_or(JsValue::Undefined);
        let on_err = args.get(1).cloned().unwrap_or(JsValue::Undefined);
        match state.borrow().clone() {
            state::PromiseState::Fulfilled(value) => Ok(reaction::settled_then(
                on_ok,
                on_err,
                state::PromiseState::Fulfilled(value),
            )),
            state::PromiseState::Rejected(reason) => Ok(reaction::settled_then(
                on_ok,
                on_err,
                state::PromiseState::Rejected(reason),
            )),
            state::PromiseState::Pending => {
                Ok(reaction::push_then(reactions.clone(), on_ok, on_err))
            }
        }
    })
}

pub(super) fn settle_reaction(
    ok: JsValue,
    err: JsValue,
    current: state::PromiseState,
    state: Rc<RefCell<state::PromiseState>>,
    object: Object,
    queue: reaction::Queue,
) {
    settle::reaction(ok, err, current, state, object, queue);
}
