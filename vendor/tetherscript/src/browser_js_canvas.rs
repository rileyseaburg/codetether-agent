//! Deterministic Canvas 2D host surface for browser JavaScript bindings.

use super::*;

#[path = "browser_js_canvas/color.rs"]
mod color;
#[path = "browser_js_canvas/context.rs"]
mod context;
#[path = "browser_js_canvas/context_2d.rs"]
mod context_2d;
#[path = "browser_js_canvas/context_rect.rs"]
mod context_rect;
#[path = "browser_js_canvas/dimensions.rs"]
mod dimensions;
#[path = "browser_js_canvas/geometry.rs"]
mod geometry;
#[path = "browser_js_canvas/image.rs"]
mod image;
#[path = "browser_js_canvas/pixels.rs"]
mod pixels;
#[path = "browser_js_canvas/store.rs"]
mod store;
#[path = "browser_js_canvas/surface.rs"]
mod surface;
#[path = "browser_js_canvas/surface_paint.rs"]
mod surface_paint;
#[path = "browser_js_canvas/webgl.rs"]
mod webgl;

pub(super) fn get_context(handle: DomHandle) -> JsValue {
    context::get_context(handle)
}

pub(super) fn dimension_value(value: Option<&JsValue>) -> u32 {
    dimensions::dimension_value(value)
}

pub(super) fn dimensions(handle: &DomHandle) -> (u32, u32) {
    dimensions::dimensions(handle)
}

pub(super) fn is_dimension_attr(handle: &DomHandle, name: &str) -> bool {
    dimensions::is_dimension_attr(handle, name)
}

pub(super) fn set_dimension(handle: &DomHandle, name: &str, value: u32) {
    dimensions::set_dimension(handle, name, value);
}

pub(super) fn reset_all() {
    store::reset_all();
    webgl::reset_all();
}

pub(super) fn reset_surface(handle: &DomHandle) {
    store::reset_surface(handle);
}
