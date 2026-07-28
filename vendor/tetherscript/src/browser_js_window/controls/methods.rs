//! Shared native method builders for window-like objects.

use super::*;

pub(super) fn flag(
    name: &'static str,
    target: &WindowObject,
    key: &'static str,
    value: bool,
) -> JsValue {
    let target = Rc::downgrade(target);
    native(name, Some(0), move |_| {
        if let Some(target) = target.upgrade() {
            target.borrow_mut().insert(key.into(), JsValue::Bool(value));
        }
        Ok(JsValue::Undefined)
    })
}

pub(super) fn noop(name: &'static str) -> JsValue {
    native(name, None, |_| Ok(JsValue::Undefined))
}

pub(super) fn install_popup(target: &WindowObject, fields: &mut HashMap<String, JsValue>) {
    fields.insert("close".into(), flag("popup.close", target, "closed", true));
    fields.insert(
        "focus".into(),
        flag("popup.focus", target, "__focused", true),
    );
    fields.insert(
        "blur".into(),
        flag("popup.blur", target, "__focused", false),
    );
    fields.insert("postMessage".into(), noop("popup.postMessage"));
}
