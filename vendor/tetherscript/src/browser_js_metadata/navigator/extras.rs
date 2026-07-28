use super::*;
use std::cell::RefCell;
use std::rc::Rc;

const UNSUPPORTED: &str = "unsupported credential operation";

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert(
        "getGamepads".into(),
        native("navigator.getGamepads", Some(0), |_| Ok(empty_array())),
    );
    navigator.insert("credentials".into(), object(credentials()));
}

fn credentials() -> HashMap<String, JsValue> {
    let mut credentials = HashMap::new();
    for method in ["get", "create", "store"] {
        credentials.insert(method.into(), rejected_method(method));
    }
    credentials.insert(
        "preventSilentAccess".into(),
        native("navigator.credentials.preventSilentAccess", Some(0), |_| {
            Ok(thenable::resolved(JsValue::Undefined))
        }),
    );
    credentials
}

fn rejected_method(method: &'static str) -> JsValue {
    let name = format!("navigator.credentials.{method}");
    let reason = format!("{name}: {UNSUPPORTED}");
    native(&name, None, move |_| {
        Ok(rejection::thenable(JsValue::String(reason.clone())))
    })
}

fn empty_array() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(Vec::new())))
}

fn object(map: HashMap<String, JsValue>) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(map)))
}
