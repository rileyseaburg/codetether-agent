use super::super::*;
use super::props;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    install_show(obj, handle);
    install_hide(obj, handle);
    install_toggle(obj, handle);
}

fn install_show(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "showPopover".into(),
        native("HTMLElement.showPopover", Some(0), move |_| {
            props::set_state(&h, true)?;
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_hide(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "hidePopover".into(),
        native("HTMLElement.hidePopover", Some(0), move |_| {
            props::set_state(&h, false)?;
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_toggle(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "togglePopover".into(),
        native("HTMLElement.togglePopover", None, move |args| {
            let next = args
                .first()
                .map(JsValue::truthy)
                .unwrap_or_else(|| attr_update::value(&h, "popover-open").is_none());
            props::set_state(&h, next)?;
            Ok(JsValue::Bool(next))
        }),
    );
}
