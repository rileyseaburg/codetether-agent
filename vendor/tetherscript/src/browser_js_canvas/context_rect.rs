//! Canvas rectangle methods.

use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: DomHandle,
    fill_style: Rc<RefCell<String>>,
) {
    let h = handle.clone();
    let style = fill_style;
    obj.insert(
        "fillRect".into(),
        native("CanvasRenderingContext2D.fillRect", Some(4), move |args| {
            let current_style = style.borrow().clone();
            super::store::mutate(&h, |surface| {
                surface.fill_rect(super::geometry::rect(args), &current_style)
            });
            Ok(JsValue::Undefined)
        }),
    );
    let h = handle;
    obj.insert(
        "clearRect".into(),
        native("CanvasRenderingContext2D.clearRect", Some(4), move |args| {
            super::store::mutate(&h, |surface| {
                surface.clear_rect(super::geometry::rect(args))
            });
            Ok(JsValue::Undefined)
        }),
    );
}
