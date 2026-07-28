//! Inert document policy probes for browser feature detection.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::super::super::native;

pub(super) fn object(kind: &str) -> JsValue {
    let prefix = format!("document.{kind}");
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        (
            "allowedFeatures".into(),
            method(&prefix, "allowedFeatures", empty),
        ),
        ("features".into(), method(&prefix, "features", empty)),
        (
            "allowsFeature".into(),
            method(&prefix, "allowsFeature", deny),
        ),
        (
            "getAllowlistForFeature".into(),
            method(&prefix, "getAllowlistForFeature", empty),
        ),
    ]))))
}

fn method(prefix: &str, name: &str, func: fn(&[JsValue]) -> Result<JsValue, String>) -> JsValue {
    native(&format!("{prefix}.{name}"), None, func)
}

fn empty(_: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(Rc::new(RefCell::new(Vec::new()))))
}

fn deny(_: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Bool(false))
}
