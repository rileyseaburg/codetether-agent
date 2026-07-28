use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert("permissions".into(), permissions());
}

fn permissions() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([(
        "query".into(),
        native("navigator.permissions.query", Some(1), query),
    )]))))
}

fn query(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(thenable::resolved(status(args.first())))
}

fn status(descriptor: Option<&JsValue>) -> JsValue {
    let mut status = HashMap::from([
        ("state".into(), JsValue::String("prompt".into())),
        ("name".into(), JsValue::String(name(descriptor))),
        ("onchange".into(), JsValue::Null),
    ]);
    events(&mut status);
    JsValue::Object(Rc::new(RefCell::new(status)))
}

fn name(descriptor: Option<&JsValue>) -> String {
    let Some(JsValue::Object(object)) = descriptor else {
        return String::new();
    };
    object
        .borrow()
        .get("name")
        .map(JsValue::display)
        .unwrap_or_default()
}

fn events(status: &mut HashMap<String, JsValue>) {
    for method in ["addEventListener", "removeEventListener"] {
        status.insert(method.into(), noop(method));
    }
    status.insert("dispatchEvent".into(), dispatch());
}

fn noop(method: &str) -> JsValue {
    let name = format!("PermissionStatus.{method}");
    native(&name, None, |_| Ok(JsValue::Undefined))
}

fn dispatch() -> JsValue {
    native("PermissionStatus.dispatchEvent", Some(1), |_| {
        Ok(JsValue::Bool(true))
    })
}
