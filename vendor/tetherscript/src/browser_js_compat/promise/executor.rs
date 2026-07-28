use super::*;

pub(super) fn construct(args: &[JsValue]) -> Result<JsValue, String> {
    let state = Rc::new(RefCell::new(state::PromiseState::Pending));
    let reactions = reaction::queue();
    let object = object::from_parts(state.clone(), reactions.clone());
    let resolve = settler(state.clone(), object.clone(), reactions.clone(), true);
    let reject = settler(state.clone(), object.clone(), reactions.clone(), false);
    let executor = args.first().cloned().unwrap_or(JsValue::Undefined);
    if let Err(error) =
        js::call_function_with_this(executor, JsValue::Undefined, &[resolve, reject])
    {
        reaction::settle(
            &state,
            &object,
            &reactions,
            state::PromiseState::Rejected(JsValue::String(error)),
        );
    }
    Ok(JsValue::Object(object))
}

fn settler(
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    reactions: reaction::Queue,
    fulfill: bool,
) -> JsValue {
    native("Promise.settle", Some(1), move |args| {
        let value = args.first().cloned().unwrap_or(JsValue::Undefined);
        let next = if fulfill {
            state::settle(value)
        } else {
            state::PromiseState::Rejected(value)
        };
        reaction::settle(&state, &object, &reactions, next);
        Ok(JsValue::Undefined)
    })
}
