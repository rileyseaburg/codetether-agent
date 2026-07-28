use std::cell::RefCell;
use std::rc::Rc;

use crate::js::JsValue;

use super::key::CloneKey;
use super::state::CloneState;

pub(super) fn clone(
    items: &Rc<RefCell<Vec<JsValue>>>,
    state: &mut CloneState,
) -> Result<JsValue, String> {
    let key = CloneKey::array(items);
    if let Some(value) = state.cached(key) {
        return Ok(value);
    }
    state.enter(key, "array")?;
    let items = items
        .borrow()
        .iter()
        .map(|item| super::clone::clone_value(item, state))
        .collect::<Result<Vec<_>, _>>()?;
    state.leave(key);
    let value = JsValue::Array(Rc::new(RefCell::new(items)));
    state.remember(key, value.clone());
    Ok(value)
}
