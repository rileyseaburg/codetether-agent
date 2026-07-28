use std::rc::Rc;

use super::{JsValue, NativeFunction};

pub(super) fn constructor() -> JsValue {
    let prototype = super::js_prototypes::register("String", super::js_prototypes::empty());
    JsValue::Native(Rc::new(
        NativeFunction::new("String", Some(1), |args| {
            Ok(JsValue::String(
                args.first().unwrap_or(&JsValue::Undefined).display(),
            ))
        })
        .with_property(
            "fromCharCode",
            native("String.fromCharCode", from_char_code),
        )
        .with_property(
            "fromCodePoint",
            native("String.fromCodePoint", from_code_point),
        )
        .with_property("prototype", prototype),
    ))
}

fn from_char_code(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::String(
        args.iter()
            .filter_map(|value| char::from_u32(value.number() as u32 & 0xffff))
            .collect(),
    ))
}

fn from_code_point(args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = String::new();
    for value in args {
        let code = value.number() as u32;
        let ch = char::from_u32(code).ok_or_else(|| format!("invalid code point {code}"))?;
        out.push(ch);
    }
    Ok(JsValue::String(out))
}

fn native(name: &'static str, func: fn(&[JsValue]) -> Result<JsValue, String>) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, None, func)))
}
