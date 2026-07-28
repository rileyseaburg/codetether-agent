//! WebGL extension metadata.

use super::*;

pub(super) const SUPPORTED: &[&str] = &[
    "ANGLE_instanced_arrays",
    "OES_element_index_uint",
    "OES_standard_derivatives",
    "OES_texture_float",
    "WEBGL_debug_renderer_info",
];

pub(super) fn supported_array() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        SUPPORTED
            .iter()
            .map(|name| JsValue::String((*name).into()))
            .collect(),
    )))
}

pub(super) fn extension_object(name: &str) -> JsValue {
    if !SUPPORTED.contains(&name) {
        return JsValue::Null;
    }
    let mut obj = HashMap::new();
    obj.insert("name".into(), JsValue::String(name.into()));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
