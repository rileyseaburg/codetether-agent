use super::*;

#[path = "js_uri/percent.rs"]
mod percent;

pub(super) fn install(env: &EnvRef) {
    for (name, decode_reserved) in [("decodeURI", true), ("decodeURIComponent", false)] {
        env.borrow_mut()
            .define(name, native_decode(name, decode_reserved));
    }
    for (name, keep_reserved) in [("encodeURI", true), ("encodeURIComponent", false)] {
        env.borrow_mut()
            .define(name, native_encode(name, keep_reserved));
    }
}

fn native_decode(name: &'static str, keep_reserved: bool) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(1), move |args| {
        Ok(JsValue::String(percent::decode(
            &args.first().unwrap_or(&JsValue::Undefined).display(),
            keep_reserved,
        )))
    })))
}

fn native_encode(name: &'static str, keep_reserved: bool) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, Some(1), move |args| {
        Ok(JsValue::String(percent::encode(
            &args.first().unwrap_or(&JsValue::Undefined).display(),
            keep_reserved,
        )))
    })))
}
