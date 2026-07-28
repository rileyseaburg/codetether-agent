//! WebGL context object construction.

use super::*;

pub(super) fn context_object(handle: DomHandle, version: u8) -> JsValue {
    super::webgl_store::ensure(&handle, version);
    let mut obj = HashMap::new();
    install_constants(&mut obj);
    install_metadata(&mut obj, handle.clone(), version);
    super::webgl_methods::install(&mut obj, handle, version);
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn install_constants(obj: &mut HashMap<String, JsValue>) {
    for (name, value) in super::webgl_constants::NAMED {
        obj.insert((*name).into(), JsValue::Number(*value as f64));
    }
}

fn install_metadata(obj: &mut HashMap<String, JsValue>, handle: DomHandle, version: u8) {
    let h = handle.clone();
    obj.insert(
        "getParameter".into(),
        native("WebGLRenderingContext.getParameter", Some(1), move |args| {
            let param = args.first().unwrap_or(&JsValue::Undefined).clone();
            Ok(super::webgl_store::with_state(&h, version, |state| {
                super::webgl_params::get(state, &param)
            }))
        }),
    );
    obj.insert(
        "getSupportedExtensions".into(),
        native(
            "WebGLRenderingContext.getSupportedExtensions",
            Some(0),
            |_| Ok(super::webgl_ext::supported_array()),
        ),
    );
    obj.insert(
        "getExtension".into(),
        native("WebGLRenderingContext.getExtension", Some(1), |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(super::webgl_ext::extension_object(&name))
        }),
    );
}
