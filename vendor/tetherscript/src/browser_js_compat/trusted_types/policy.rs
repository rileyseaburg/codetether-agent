use super::*;

const KINDS: [&str; 3] = ["HTML", "Script", "ScriptURL"];

pub(super) fn function() -> JsValue {
    native("trustedTypes.createPolicy", None, create)
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    let name = args.first().map(JsValue::display).unwrap_or_default();
    let rules = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    let mut obj = HashMap::from([("name".into(), JsValue::String(name))]);
    for kind in KINDS {
        obj.insert(format!("create{kind}"), method(&rules, kind));
    }
    Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
}

fn method(rules: &JsValue, kind: &str) -> JsValue {
    let rule = rule(rules, &format!("create{kind}"));
    native(
        &format!("TrustedTypePolicy.create{kind}"),
        None,
        move |args| {
            let input = args.first().cloned().unwrap_or(JsValue::Undefined);
            let output = match &rule {
                Some(rule) => call_rule(rule, input)?,
                None => input,
            };
            Ok(JsValue::String(output.display()))
        },
    )
}

fn call_rule(rule: &JsValue, input: JsValue) -> Result<JsValue, String> {
    js::call_function_with_this(rule.clone(), JsValue::Undefined, &[input])
}

fn rule(rules: &JsValue, name: &str) -> Option<JsValue> {
    match rules {
        JsValue::Object(obj) => obj.borrow().get(name).cloned().filter(callable),
        _ => None,
    }
}

fn callable(value: &JsValue) -> bool {
    matches!(
        value,
        JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Native(_)
    )
}
