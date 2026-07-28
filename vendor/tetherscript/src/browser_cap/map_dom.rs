//! DOM, keyboard, pointer, upload, and screenshot mappings.

use crate::value::Value;

use super::call::BrowserCall;

#[path = "map_dom_basic.rs"]
mod basic;
#[path = "map_dom_more.rs"]
mod more;
#[path = "map_dom_motion.rs"]
mod motion;

pub(crate) fn prepare(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "click" | "hover" | "focus" | "blur" | "fill_native" | "toggle" => {
            basic::selector_action(method, args)
        }
        "upload" => more::upload(args),
        "fill" => basic::pair("fill", args, "selector", "value"),
        "type" => basic::pair("type", args, "selector", "text"),
        "press" | "keyboard_press" => basic::one(method, args, "key"),
        "click_text" => more::click_text(args),
        "scroll" => motion::scroll(args),
        "mouse_click" => more::xy(args),
        "keyboard_type" => basic::one("keyboard_type", args, "text"),
        _ => unreachable!(),
    }
}
