//! Visual and element-state mappings implemented via browserctl eval.

use crate::value::Value;

use super::call::BrowserCall;

#[path = "map_visual_js.rs"]
mod js;
#[path = "map_visual_actions.rs"]
mod visual_actions;
#[path = "map_visual_more.rs"]
mod visual_more;

pub(crate) fn prepare(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "is_visible" => visual_actions::selector_eval(method, args, js::visible),
        "is_enabled" => visual_actions::selector_eval(method, args, js::enabled),
        "bounding_box" => visual_actions::selector_eval(method, args, js::bounding_box),
        "screenshot_element" => visual_actions::screenshot_element(args),
        "find_visual_text" => visual_actions::text_wait(args),
        "find_element_at" => visual_more::point_eval(args),
        "compare_screenshots" | "visual_diff" | "assert_screenshot_matches" => {
            visual_more::raw_visual(method, args)
        }
        _ => unreachable!(),
    }
}
