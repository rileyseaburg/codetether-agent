use super::*;

pub(super) fn install_number() {
    insert("Number", "valueOf", number_value_of);
}

pub(super) fn install_boolean() {
    insert("Boolean", "valueOf", boolean_value_of);
}

fn insert(name: &'static str, prop: &'static str, func: fn(&[JsValue]) -> Result<JsValue, String>) {
    let Some(JsValue::Object(prototype)) = js_prototypes::get(name) else {
        return;
    };
    prototype.borrow_mut().insert(
        prop.into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            format!("{name}.prototype.{prop}"),
            Some(1),
            func,
        ))),
    );
}

fn number_value_of(args: &[JsValue]) -> Result<JsValue, String> {
    match &args[0] {
        JsValue::Number(number) => Ok(JsValue::Number(*number)),
        _ => Err("TypeError: Number.prototype.valueOf requires Number".into()),
    }
}

fn boolean_value_of(args: &[JsValue]) -> Result<JsValue, String> {
    match &args[0] {
        JsValue::Bool(value) => Ok(JsValue::Bool(*value)),
        _ => Err("TypeError: Boolean.prototype.valueOf requires Boolean".into()),
    }
}
