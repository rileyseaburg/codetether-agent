//! JavaScript snippets for pointer actions.

use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::interact::pointer_event_fields as fields;
use crate::browser_agent::keyboard_escape::node;

pub(crate) fn hover(path: &[usize], bounds: BoundingBox) -> String {
    format!(
        "let n={}; n.dispatchEvent({{type:'mouseover',{}}}); \
         n.dispatchEvent({{type:'mouseenter',bubbles:false,{}}}); \
         n.dispatchEvent({{type:'mousemove',{}}});",
        node(path),
        fields::mouse(0, 0, bounds),
        fields::mouse(0, 0, bounds),
        fields::mouse(0, 0, bounds)
    )
}

pub(crate) fn mouse_down(path: &[usize], bounds: BoundingBox) -> String {
    dispatch(path, "mousedown", 1, 1, bounds)
}

pub(crate) fn mouse_up(path: &[usize], bounds: BoundingBox) -> String {
    dispatch(path, "mouseup", 0, 1, bounds)
}

pub(crate) fn double_click(path: &[usize], bounds: BoundingBox) -> String {
    format!(
        "{}{}{}",
        click_once(path, 1, bounds),
        click_once(path, 2, bounds),
        dispatch(path, "dblclick", 0, 2, bounds)
    )
}

fn click_once(path: &[usize], detail: i64, bounds: BoundingBox) -> String {
    format!(
        "{}{}{}.dispatchEvent({{type:'click',{}}});",
        dispatch(path, "mousedown", 1, detail, bounds),
        dispatch(path, "mouseup", 0, detail, bounds),
        node(path),
        fields::mouse(0, detail, bounds)
    )
}

fn dispatch(path: &[usize], event: &str, buttons: i64, detail: i64, bounds: BoundingBox) -> String {
    format!(
        "{}.dispatchEvent({{type:'{event}',{}}});",
        node(path),
        fields::mouse(buttons, detail, bounds)
    )
}
