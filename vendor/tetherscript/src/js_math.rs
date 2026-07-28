use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{EnvRef, JsValue};

#[path = "js_math_ops.rs"]
mod js_math_ops;

use self::js_math_ops::{binary, fold, random, unary};

pub(super) fn install(env: &EnvRef) {
    let mut obj = HashMap::new();
    obj.insert("E".into(), JsValue::Number(std::f64::consts::E));
    obj.insert("PI".into(), JsValue::Number(std::f64::consts::PI));
    obj.insert("LN10".into(), JsValue::Number(std::f64::consts::LN_10));
    obj.insert("LN2".into(), JsValue::Number(std::f64::consts::LN_2));
    obj.insert("LOG10E".into(), JsValue::Number(std::f64::consts::LOG10_E));
    obj.insert("LOG2E".into(), JsValue::Number(std::f64::consts::LOG2_E));
    obj.insert(
        "SQRT1_2".into(),
        JsValue::Number(std::f64::consts::FRAC_1_SQRT_2),
    );
    obj.insert("SQRT2".into(), JsValue::Number(std::f64::consts::SQRT_2));
    obj.insert("abs".into(), unary("Math.abs", f64::abs));
    obj.insert("ceil".into(), unary("Math.ceil", f64::ceil));
    obj.insert("floor".into(), unary("Math.floor", f64::floor));
    obj.insert("round".into(), unary("Math.round", f64::round));
    obj.insert("sqrt".into(), unary("Math.sqrt", f64::sqrt));
    obj.insert("log".into(), unary("Math.log", f64::ln));
    obj.insert("log2".into(), unary("Math.log2", f64::log2));
    obj.insert("log10".into(), unary("Math.log10", f64::log10));
    obj.insert("exp".into(), unary("Math.exp", f64::exp));
    obj.insert("sin".into(), unary("Math.sin", f64::sin));
    obj.insert("cos".into(), unary("Math.cos", f64::cos));
    obj.insert("tan".into(), unary("Math.tan", f64::tan));
    obj.insert("asin".into(), unary("Math.asin", f64::asin));
    obj.insert("acos".into(), unary("Math.acos", f64::acos));
    obj.insert("atan".into(), unary("Math.atan", f64::atan));
    obj.insert("trunc".into(), unary("Math.trunc", f64::trunc));
    obj.insert("sign".into(), unary("Math.sign", f64::signum));
    obj.insert("max".into(), fold("Math.max", f64::NEG_INFINITY, f64::max));
    obj.insert("min".into(), fold("Math.min", f64::INFINITY, f64::min));
    obj.insert("pow".into(), binary("Math.pow", f64::powf));
    obj.insert("atan2".into(), binary("Math.atan2", f64::atan2));
    obj.insert("random".into(), random());
    env.borrow_mut()
        .define("Math", JsValue::Object(Rc::new(RefCell::new(obj))));
}
