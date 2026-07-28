use super::super::*;

pub(super) enum Action {
    Ready(state::PromiseState),
    Wait(JsValue),
}

pub(super) fn from_handlers(ok: JsValue, err: JsValue, current: state::PromiseState) -> Action {
    match current {
        state::PromiseState::Fulfilled(value) => fulfilled(ok, value),
        state::PromiseState::Rejected(reason) => rejected(err, reason),
        state::PromiseState::Pending => Action::Ready(state::PromiseState::Pending),
    }
}

fn fulfilled(handler_value: JsValue, value: JsValue) -> Action {
    if handler::present(&handler_value) {
        handled(handler_value, &[value])
    } else {
        Action::Ready(state::PromiseState::Fulfilled(value))
    }
}

fn rejected(handler_value: JsValue, reason: JsValue) -> Action {
    if handler::present(&handler_value) {
        handled(handler_value, &[reason])
    } else {
        Action::Ready(state::PromiseState::Rejected(reason))
    }
}

fn handled(handler_value: JsValue, args: &[JsValue]) -> Action {
    match js::call_function_with_this(handler_value, JsValue::Undefined, args) {
        Ok(value) => match state::from_value(&value) {
            Some(state::PromiseState::Pending) => Action::Wait(value),
            Some(state) => Action::Ready(state),
            None => Action::Ready(state::PromiseState::Fulfilled(value)),
        },
        Err(error) => Action::Ready(state::PromiseState::Rejected(JsValue::String(error))),
    }
}
