use super::*;
use std::{cell::RefCell, rc::Rc};

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    let connection = navigator
        .get("connection")
        .cloned()
        .unwrap_or_else(network_information);
    enrich_network_information(&connection);
    navigator.insert("connection".into(), connection.clone());
    navigator.insert("networkInformation".into(), connection);
}

fn network_information() -> JsValue {
    let value = JsValue::Object(Rc::new(RefCell::new(HashMap::new())));
    enrich_network_information(&value);
    value
}

fn enrich_network_information(value: &JsValue) {
    let JsValue::Object(object) = value else {
        return;
    };
    let mut info = object.borrow_mut();
    for (name, value) in [("effectiveType", "4g"), ("type", "wifi")] {
        info.entry(name.into())
            .or_insert(JsValue::String(value.into()));
    }
    for (name, value) in [("downlink", 10.0), ("downlinkMax", 10.0), ("rtt", 50.0)] {
        info.entry(name.into()).or_insert(JsValue::Number(value));
    }
    info.entry("saveData".into())
        .or_insert(JsValue::Bool(false));
    info.entry("onchange".into()).or_insert(JsValue::Null);
    events(&mut info);
}

fn events(info: &mut HashMap<String, JsValue>) {
    for method in ["addEventListener", "removeEventListener"] {
        info.entry(method.into())
            .or_insert_with(|| noop_event_method(method));
    }
    info.entry("dispatchEvent".into())
        .or_insert_with(dispatch_event);
}

fn noop_event_method(method: &str) -> JsValue {
    let name = format!("NetworkInformation.{method}");
    native(&name, None, |_| Ok(JsValue::Undefined))
}

fn dispatch_event() -> JsValue {
    native("NetworkInformation.dispatchEvent", None, |_| {
        Ok(JsValue::Bool(true))
    })
}
