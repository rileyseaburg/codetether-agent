//! Canvas context dispatch.

use super::*;

pub(super) fn get_context(handle: DomHandle) -> JsValue {
    native("HTMLCanvasElement.getContext", Some(1), move |args| {
        let kind = args.first().unwrap_or(&JsValue::Undefined).display();
        match kind.as_str() {
            "2d" => Ok(super::context_2d::context_object(handle.clone())),
            "webgl" | "experimental-webgl" => Ok(super::webgl::context_object(handle.clone(), 1)),
            "webgl2" => Ok(super::webgl::context_object(handle.clone(), 2)),
            _ => Ok(JsValue::Null),
        }
    })
}
