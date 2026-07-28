use super::*;

#[path = "js_date/civil.rs"]
mod civil;
#[path = "js_date/clock.rs"]
mod clock;
#[path = "js_date/field.rs"]
mod field;
#[path = "js_date/methods.rs"]
mod methods;
#[path = "js_date/object.rs"]
mod object;
#[path = "js_date/parts.rs"]
mod parts;
#[path = "js_date/state.rs"]
mod state;

pub(super) fn constructor() -> JsValue {
    JsValue::Native(Rc::new(
        NativeFunction::new("Date", None, |args| Ok(object::new(arg_ms(args.first()))))
            .with_property(
                "now",
                JsValue::Native(Rc::new(NativeFunction::new("Date.now", Some(0), |_| {
                    Ok(JsValue::Number(clock::now_ms()))
                }))),
            )
            .with_property(
                "parse",
                JsValue::Native(Rc::new(NativeFunction::new(
                    "Date.parse",
                    Some(1),
                    |args| Ok(JsValue::Number(arg_ms(args.first()))),
                ))),
            ),
    ))
}

fn arg_ms(value: Option<&JsValue>) -> f64 {
    value.map_or_else(clock::now_ms, JsValue::number)
}
