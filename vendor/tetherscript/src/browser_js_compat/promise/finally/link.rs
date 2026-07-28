use super::super::super::*;
use super::callbacks;

pub(super) fn attach(
    returned: &JsValue,
    original: state::PromiseState,
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    queue: reaction::Queue,
) -> Result<(), String> {
    let ok = callbacks::resume(state.clone(), object.clone(), queue.clone(), original);
    let err = callbacks::reject(state, object, queue);
    let JsValue::Object(promise) = returned else {
        return Ok(());
    };
    let Some(then) = promise.borrow().get("then").cloned() else {
        return Ok(());
    };
    js::call_function_with_this(then, returned.clone(), &[ok, err]).map(|_| ())
}
