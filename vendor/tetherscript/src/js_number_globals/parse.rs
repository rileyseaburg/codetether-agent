use super::*;

#[path = "parse/float.rs"]
mod float;
#[path = "parse/int.rs"]
mod int;

pub(super) fn install(env: &EnvRef) {
    env.borrow_mut().define(
        "parseInt",
        JsValue::Native(Rc::new(NativeFunction::new("parseInt", None, |args| {
            Ok(int::parse(args))
        }))),
    );
    env.borrow_mut().define("parseFloat", float::native());
}
