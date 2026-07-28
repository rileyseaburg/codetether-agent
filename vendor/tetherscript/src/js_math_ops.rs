use std::{cell::RefCell, rc::Rc};

use super::super::{JsValue, NativeFunction};

pub(super) fn unary(name: &'static str, op: fn(f64) -> f64) -> JsValue {
    native(name, Some(1), move |args| {
        let value = args.first().unwrap_or(&JsValue::Undefined);
        Ok(JsValue::Number(op(value.number())))
    })
}

pub(super) fn binary(name: &'static str, op: fn(f64, f64) -> f64) -> JsValue {
    native(name, Some(2), move |args| {
        Ok(JsValue::Number(op(args[0].number(), args[1].number())))
    })
}

pub(super) fn fold(name: &'static str, init: f64, op: fn(f64, f64) -> f64) -> JsValue {
    native(name, None, move |args| {
        let value = args.iter().fold(init, |acc, value| op(acc, value.number()));
        Ok(JsValue::Number(value))
    })
}

pub(super) fn random() -> JsValue {
    let seed = Rc::new(RefCell::new(0x4d595df4d0f33173u64));
    native("Math.random", Some(0), move |_| {
        let mut state = seed.borrow_mut();
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        Ok(JsValue::Number((*state >> 11) as f64 / (1u64 << 53) as f64))
    })
}

fn native(
    name: &str,
    arity: Option<usize>,
    func: impl Fn(&[JsValue]) -> Result<JsValue, String> + 'static,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, arity, func)))
}
