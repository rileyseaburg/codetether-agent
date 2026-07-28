use super::super::*;

pub(super) enum Action {
    Ready(state::PromiseState),
    Wait(JsValue, state::PromiseState),
}

pub(super) fn after_callback(callback: JsValue, current: state::PromiseState) -> Action {
    let returned = match js::call_function_with_this(callback, JsValue::Undefined, &[]) {
        Ok(value) => value,
        Err(error) => return Action::Ready(state::PromiseState::Rejected(JsValue::String(error))),
    };
    match state::settle(returned.clone()) {
        state::PromiseState::Rejected(reason) => {
            Action::Ready(state::PromiseState::Rejected(reason))
        }
        state::PromiseState::Pending => Action::Wait(returned, current),
        _ => Action::Ready(current),
    }
}
