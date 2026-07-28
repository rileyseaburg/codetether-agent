//! WebGL stateful methods.

use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: DomHandle, version: u8) {
    let h = handle.clone();
    obj.insert(
        "viewport".into(),
        native("WebGLRenderingContext.viewport", Some(4), move |args| {
            let viewport = super::webgl_values::i64_quad(args);
            super::webgl_store::mutate(&h, version, |state| {
                state.viewport = viewport;
                state.push(format!(
                    "viewport|{}|{}|{}|{}",
                    viewport[0], viewport[1], viewport[2], viewport[3]
                ));
            });
            Ok(JsValue::Undefined)
        }),
    );
    install_clear_color(obj, handle.clone(), version);
    install_clear(obj, handle, version);
}

fn install_clear_color(obj: &mut HashMap<String, JsValue>, handle: DomHandle, version: u8) {
    obj.insert(
        "clearColor".into(),
        native("WebGLRenderingContext.clearColor", Some(4), move |args| {
            let color = super::webgl_values::f64_quad(args);
            super::webgl_store::mutate(&handle, version, |state| {
                state.clear_color = color;
                state.push(format!(
                    "clearColor|{}|{}|{}|{}",
                    color[0], color[1], color[2], color[3]
                ));
            });
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_clear(obj: &mut HashMap<String, JsValue>, handle: DomHandle, version: u8) {
    obj.insert(
        "clear".into(),
        native("WebGLRenderingContext.clear", Some(1), move |args| {
            let mask = super::webgl_values::i64_value(args.first());
            super::webgl_store::mutate(&handle, version, |state| {
                state.push(format!("clear|{}", mask));
            });
            Ok(JsValue::Undefined)
        }),
    );
}
