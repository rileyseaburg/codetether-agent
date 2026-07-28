//! WebGL metadata host modules.

use super::*;

#[path = "webgl_attrs.rs"]
mod webgl_attrs;
#[path = "webgl_constants.rs"]
mod webgl_constants;
#[path = "webgl_context.rs"]
mod webgl_context;
#[path = "webgl_ext.rs"]
mod webgl_ext;
#[path = "webgl_methods.rs"]
mod webgl_methods;
#[path = "webgl_params.rs"]
mod webgl_params;
#[path = "webgl_state.rs"]
mod webgl_state;
#[path = "webgl_store.rs"]
mod webgl_store;
#[path = "webgl_values.rs"]
mod webgl_values;

pub(super) fn context_object(handle: DomHandle, version: u8) -> JsValue {
    webgl_context::context_object(handle, version)
}

pub(super) fn reset_all() {
    webgl_store::reset_all();
}
