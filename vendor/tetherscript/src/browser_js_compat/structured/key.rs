use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct CloneKey {
    kind: u8,
    id: usize,
}

impl CloneKey {
    pub(super) fn array(value: &Rc<RefCell<Vec<JsValue>>>) -> Self {
        Self {
            kind: b'a',
            id: Rc::as_ptr(value) as usize,
        }
    }

    pub(super) fn object(value: &Rc<RefCell<HashMap<String, JsValue>>>) -> Self {
        Self {
            kind: b'o',
            id: Rc::as_ptr(value) as usize,
        }
    }
}
