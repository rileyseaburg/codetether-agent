use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::native;
use super::rules;
use super::state::Cssom;

pub(super) fn object(cssom: Cssom, sheet: usize) -> JsValue {
    let rule_array = rules::array(&cssom, sheet);
    let mut obj = HashMap::new();
    obj.insert("cssRules".into(), JsValue::Array(rule_array.clone()));
    obj.insert(
        "insertRule".into(),
        insert_rule(cssom.clone(), sheet, rule_array.clone()),
    );
    obj.insert("deleteRule".into(), delete_rule(cssom, sheet, rule_array));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn insert_rule(cssom: Cssom, sheet: usize, array: Rc<RefCell<Vec<JsValue>>>) -> JsValue {
    native("CSSStyleSheet.insertRule", None, move |args| {
        let source = args.first().unwrap_or(&JsValue::Undefined).display();
        let index = cssom.insert_rule(sheet, &source, args.get(1))?;
        rules::refresh(&array, &cssom, sheet);
        Ok(JsValue::Number(index as f64))
    })
}

fn delete_rule(cssom: Cssom, sheet: usize, array: Rc<RefCell<Vec<JsValue>>>) -> JsValue {
    native("CSSStyleSheet.deleteRule", Some(1), move |args| {
        cssom.delete_rule(sheet, args.first().unwrap_or(&JsValue::Undefined))?;
        rules::refresh(&array, &cssom, sheet);
        Ok(JsValue::Undefined)
    })
}
