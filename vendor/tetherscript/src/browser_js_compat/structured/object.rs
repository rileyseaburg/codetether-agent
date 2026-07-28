use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::key::CloneKey;
use super::state::CloneState;

pub(super) fn clone(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &mut CloneState,
) -> Result<JsValue, String> {
    let key = CloneKey::object(object);
    if let Some(value) = state.cached(key) {
        return Ok(value);
    }
    state.enter(key, "object")?;
    let mut cloned = HashMap::new();
    for (name, value) in object.borrow().iter() {
        cloned.insert(name.clone(), super::clone::clone_value(value, state)?);
    }
    state.leave(key);
    let value = JsValue::Object(Rc::new(RefCell::new(cloned)));
    state.remember(key, value.clone());
    Ok(value)
}
