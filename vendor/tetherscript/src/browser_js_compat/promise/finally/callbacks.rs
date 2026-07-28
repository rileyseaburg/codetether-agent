use super::super::super::*;

pub(super) fn resume(
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    queue: reaction::Queue,
    original: state::PromiseState,
) -> JsValue {
    native("Promise.finally.resume", Some(1), move |_| {
        reaction::settle(&state, &object, &queue, original.clone());
        Ok(JsValue::Undefined)
    })
}

pub(super) fn reject(
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    queue: reaction::Queue,
) -> JsValue {
    native("Promise.finally.reject", Some(1), move |args| {
        let reason = args.first().cloned().unwrap_or(JsValue::Undefined);
        reaction::settle(
            &state,
            &object,
            &queue,
            state::PromiseState::Rejected(reason),
        );
        Ok(JsValue::Undefined)
    })
}
