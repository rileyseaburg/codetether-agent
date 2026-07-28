use super::*;

#[path = "trusted_types/policy.rs"]
mod policy;

#[cfg(test)]
#[path = "trusted_types/tests.rs"]
mod tests;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert("trustedTypes".into(), object());
}

fn object() -> JsValue {
    let mut obj = HashMap::from([
        ("emptyHTML".into(), JsValue::String(String::new())),
        ("emptyScript".into(), JsValue::String(String::new())),
        ("createPolicy".into(), policy::function()),
    ]);
    for kind in ["HTML", "Script", "ScriptURL"] {
        obj.insert(format!("is{kind}"), probe(kind));
    }
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn probe(kind: &'static str) -> JsValue {
    native(&format!("trustedTypes.is{kind}"), Some(1), |_| {
        Ok(JsValue::Bool(false))
    })
}
