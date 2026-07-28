//! Deterministic probe methods for the top-level window object.

use super::*;

pub(super) fn install(window: &WindowObject) {
    let mut obj = window.borrow_mut();
    obj.insert("__printCount".into(), JsValue::Number(0.0));
    obj.insert("__stopped".into(), JsValue::Bool(false));
    obj.insert("__focused".into(), JsValue::Bool(true));
    obj.insert(
        "close".into(),
        methods::flag("window.close", window, "closed", true),
    );
    obj.insert(
        "stop".into(),
        methods::flag("window.stop", window, "__stopped", true),
    );
    obj.insert(
        "focus".into(),
        methods::flag("window.focus", window, "__focused", true),
    );
    obj.insert(
        "blur".into(),
        methods::flag("window.blur", window, "__focused", false),
    );
    obj.insert("print".into(), print_method(window));
}

fn print_method(window: &WindowObject) -> JsValue {
    let target = Rc::downgrade(window);
    native("window.print", Some(0), move |_| {
        if let Some(target) = target.upgrade() {
            let next = print_count(&target) + 1.0;
            target
                .borrow_mut()
                .insert("__printCount".into(), JsValue::Number(next));
        }
        Ok(JsValue::Undefined)
    })
}

fn print_count(window: &WindowObject) -> f64 {
    match window.borrow().get("__printCount") {
        Some(JsValue::Number(count)) => *count,
        _ => 0.0,
    }
}
