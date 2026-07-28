use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::native;
use super::sheet_object;
use super::state::Cssom;

pub(super) fn object(cssom: &Cssom) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("length".into(), JsValue::Number(cssom.len() as f64));
    for index in 0..cssom.len() {
        obj.insert(
            index.to_string(),
            sheet_object::object(cssom.clone(), index),
        );
    }
    let item_cssom = cssom.clone();
    obj.insert(
        "item".into(),
        native("StyleSheetList.item", Some(1), move |args| {
            item(args, &item_cssom)
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn item(args: &[JsValue], cssom: &Cssom) -> Result<JsValue, String> {
    let index = args
        .first()
        .unwrap_or(&JsValue::Undefined)
        .display()
        .parse::<usize>()
        .unwrap_or(usize::MAX);
    if index >= cssom.len() {
        Ok(JsValue::Null)
    } else {
        Ok(sheet_object::object(cssom.clone(), index))
    }
}
