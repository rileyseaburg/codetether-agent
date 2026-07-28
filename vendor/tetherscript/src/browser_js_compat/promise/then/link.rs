use super::super::*;
use super::Object;

type SharedState = Rc<RefCell<state::PromiseState>>;

pub(super) fn attach(
    value: &JsValue,
    state: SharedState,
    object: Object,
    queue: reaction::Queue,
) -> Result<(), String> {
    let then = js::get_host_property(value, "then")?;
    if matches!(then, JsValue::Undefined | JsValue::Null) {
        reaction::settle(
            &state,
            &object,
            &queue,
            state::PromiseState::Fulfilled(value.clone()),
        );
        return Ok(());
    }
    js::call_function_with_this(
        then,
        value.clone(),
        &[
            settler(state.clone(), object.clone(), queue.clone(), true),
            settler(state, object, queue, false),
        ],
    )?;
    Ok(())
}

fn settler(state: SharedState, object: Object, queue: reaction::Queue, ok: bool) -> JsValue {
    let name = if ok {
        "Promise.adopt"
    } else {
        "Promise.adopt.reject"
    };
    native(name, Some(1), move |args| {
        let value = args.first().cloned().unwrap_or(JsValue::Undefined);
        let next = if ok {
            state::settle(value)
        } else {
            state::PromiseState::Rejected(value)
        };
        reaction::settle(&state, &object, &queue, next);
        Ok(JsValue::Undefined)
    })
}
