use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::index;
use super::model::Rule;
use super::native;
use super::parse_rule;
use super::rule_object;

type RuleStore = Rc<RefCell<Vec<Rule>>>;
type RuleArray = Rc<RefCell<Vec<JsValue>>>;

pub(super) fn constructor() -> JsValue {
    native("CSSStyleSheet", Some(0), move |_| Ok(object()))
}

fn object() -> JsValue {
    let rules = Rc::new(RefCell::new(Vec::new()));
    let array = Rc::new(RefCell::new(Vec::new()));
    let mut obj = HashMap::new();
    obj.insert("cssRules".into(), JsValue::Array(array.clone()));
    obj.insert(
        "insertRule".into(),
        insert_rule(rules.clone(), array.clone()),
    );
    obj.insert("deleteRule".into(), delete_rule(rules, array));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn insert_rule(rules: RuleStore, array: RuleArray) -> JsValue {
    native("CSSStyleSheet.insertRule", None, move |args| {
        let source = args.first().unwrap_or(&JsValue::Undefined).display();
        let rule = parse_rule::parse(&source)?;
        let index = index::insert(args.get(1), rules.borrow().len())?;
        rules.borrow_mut().insert(index, rule);
        refresh(&rules, &array);
        Ok(JsValue::Number(index as f64))
    })
}

fn delete_rule(rules: RuleStore, array: RuleArray) -> JsValue {
    native("CSSStyleSheet.deleteRule", Some(1), move |args| {
        let index = index::delete(
            args.first().unwrap_or(&JsValue::Undefined),
            rules.borrow().len(),
        )?;
        rules.borrow_mut().remove(index);
        refresh(&rules, &array);
        Ok(JsValue::Undefined)
    })
}

fn refresh(rules: &RuleStore, array: &RuleArray) {
    *array.borrow_mut() = rules.borrow().iter().map(rule_object::object).collect();
}
