use super::*;

pub(super) fn value() -> JsValue {
    let constructor = NativeFunction::new("Window.Promise", Some(1), executor::construct)
        .with_property("resolve", function("Promise.resolve", Some(1), resolve))
        .with_property("reject", function("Promise.reject", Some(1), reject))
        .with_property("all", function("Promise.all", Some(1), aggregate::all))
        .with_property("race", function("Promise.race", Some(1), aggregate::race));
    JsValue::Native(Rc::new(constructor))
}

fn function(
    name: &str,
    arity: Option<usize>,
    func: fn(&[JsValue]) -> Result<JsValue, String>,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, arity, func)))
}

fn resolve(args: &[JsValue]) -> Result<JsValue, String> {
    let value = args.first().cloned().unwrap_or(JsValue::Undefined);
    Ok(object::from_state(state::settle(value)))
}

fn reject(args: &[JsValue]) -> Result<JsValue, String> {
    let reason = args.first().cloned().unwrap_or(JsValue::Undefined);
    Ok(api::rejected(reason))
}
