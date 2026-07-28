use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::escape;
use super::native;
use super::supports;

pub(super) fn object() -> JsValue {
    let mut obj = HashMap::new();
    obj.insert(
        "supports".into(),
        native("CSS.supports", None, move |args| {
            Ok(JsValue::Bool(supports::args(args)))
        }),
    );
    obj.insert(
        "escape".into(),
        native("CSS.escape", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(JsValue::String(escape::ident(&value)))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
