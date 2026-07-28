use super::*;

#[path = "js_number_globals/parse.rs"]
mod parse;

pub(super) fn install(env: &EnvRef) {
    env.borrow_mut().define(
        "isNaN",
        JsValue::Native(Rc::new(NativeFunction::new("isNaN", Some(1), |args| {
            Ok(JsValue::Bool(
                args.first()
                    .unwrap_or(&JsValue::Undefined)
                    .number()
                    .is_nan(),
            ))
        }))),
    );
    env.borrow_mut().define(
        "isFinite",
        JsValue::Native(Rc::new(NativeFunction::new("isFinite", Some(1), |args| {
            Ok(JsValue::Bool(
                args.first()
                    .unwrap_or(&JsValue::Undefined)
                    .number()
                    .is_finite(),
            ))
        }))),
    );
    parse::install(env);
}
