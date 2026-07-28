use std::cell::RefCell;
use std::rc::Rc;

use crate::js::JsValue;

use super::rule_object;
use super::state::Cssom;

pub(super) fn array(cssom: &Cssom, sheet: usize) -> Rc<RefCell<Vec<JsValue>>> {
    Rc::new(RefCell::new(objects(cssom, sheet)))
}

pub(super) fn refresh(array: &Rc<RefCell<Vec<JsValue>>>, cssom: &Cssom, sheet: usize) {
    *array.borrow_mut() = objects(cssom, sheet);
}

fn objects(cssom: &Cssom, sheet: usize) -> Vec<JsValue> {
    cssom.rules(sheet).iter().map(rule_object::object).collect()
}
