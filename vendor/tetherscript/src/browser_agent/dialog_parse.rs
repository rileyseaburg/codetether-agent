//! Parse dialog records returned by the JavaScript bridge.

use crate::browser_agent::dialog::{dialog_script, DialogRecord};
use crate::js::JsValue;

pub(crate) fn captured_dialogs(value: JsValue) -> (usize, Vec<DialogRecord>) {
    let JsValue::Array(top) = value else {
        return (0, Vec::new());
    };
    let top = top.borrow();
    let consumed = top
        .first()
        .map(|value| value.display().parse().unwrap_or(0))
        .unwrap_or(0);
    let Some(JsValue::Array(items)) = top.get(1) else {
        return (consumed, Vec::new());
    };
    let items = items.clone();
    drop(top);
    let records = items.borrow().iter().filter_map(record).collect();
    (consumed, records)
}

fn record(value: &JsValue) -> Option<DialogRecord> {
    let JsValue::Array(fields) = value else {
        return None;
    };
    let fields = fields.borrow();
    Some(DialogRecord {
        sequence: 0,
        kind: dialog_script::kind(&fields.first()?.display())?,
        message: fields.get(1)?.display(),
        default_value: text_opt(fields.get(2)?),
        accepted: Some(fields.get(3)?.truthy()),
        response: text_opt(fields.get(4)?),
    })
}

fn text_opt(value: &JsValue) -> Option<String> {
    match value {
        JsValue::Null | JsValue::Undefined => None,
        value => Some(value.display()),
    }
}
