//! Console object installation for the in-tree JavaScript engine.

use super::{EnvRef, JsValue, NativeFunction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) fn install(env: &EnvRef, sink: Rc<RefCell<Vec<String>>>) {
    let mut console = HashMap::new();
    for level in ["log", "info", "debug", "warn", "error"] {
        console.insert(level.into(), method(level, sink.clone()));
    }
    env.borrow_mut()
        .define("console", JsValue::Object(Rc::new(RefCell::new(console))));
}

fn method(level: &'static str, sink: Rc<RefCell<Vec<String>>>) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(level, None, move |args| {
        let line = args
            .iter()
            .map(|value| value.display())
            .collect::<Vec<_>>()
            .join(" ");
        sink.borrow_mut().push(format_line(level, &line));
        Ok(JsValue::Undefined)
    })))
}

fn format_line(level: &str, line: &str) -> String {
    if level == "log" {
        line.into()
    } else {
        format!("[{level}] {line}")
    }
}
