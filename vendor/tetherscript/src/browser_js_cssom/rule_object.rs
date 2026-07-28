use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::decl_object;
use super::model::Rule;

pub(super) fn object(rule: &Rule) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("cssText".into(), JsValue::String(rule.css_text.clone()));
    obj.insert(
        "selectorText".into(),
        JsValue::String(rule.selector_text.clone()),
    );
    obj.insert("style".into(), decl_object::object(&rule.declarations));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
