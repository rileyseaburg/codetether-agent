//! Method dispatch for browser capability mappings.

use crate::value::Value;

use super::authority::BrowserAuthority;
use super::call::BrowserCall;

#[rustfmt::skip]
pub(crate) fn prepare(auth: &BrowserAuthority, method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "raw" => super::raw::prepare(auth, args),
        "health" | "detect" | "start" | "stop" | "goto" | "reload" | "back" | "forward"
        | "tabs" | "tabs_select" | "tabs_new" | "tabs_close" | "wait_for_url" => {
            super::nav::prepare(auth, method, args)
        }
        "click" | "upload" | "fill" | "type" | "press" | "hover" | "focus" | "blur"
        | "scroll" | "click_text" | "fill_native" | "toggle" | "mouse_click"
        | "keyboard_type" | "keyboard_press" => {
            super::dom::prepare(method, args)
        }
        "snapshot" | "page_snapshot" | "dom_snapshot" | "text" | "html" | "eval"
        | "wait_for_selector" | "wait_for_text" | "wait_for_network_idle" => {
            super::extra::prepare(method, args)
        }
        "console_logs" | "console_errors" | "unhandled_rejections" | "runtime_exceptions"
        | "source_mapped_stack_traces" | "frameworks" | "react.detect" | "react.version"
        | "react.component_tree" | "react.errors" | "react.hydration_warnings"
        | "react.suspense_boundaries" | "react.component_for_selector" | "react.props"
        | "react.state" | "react.hooks" | "react.owner_stack" => {
            super::diag::prepare(method, args)
        }
        "network_log" | "failed_requests" | "fetch" | "axios" | "xhr" | "replay"
        | "replay_request" | "diagnose" | "wait_for_request" | "wait_for_response" => {
            super::net::prepare(auth, method, args)
        }
        "cookies" | "local_storage" | "session_storage" | "indexed_db_summary"
        | "set_cookie" | "set_local_storage" | "clear_storage" => {
            super::storage::prepare(method, args)
        }
        "is_visible" | "is_enabled" | "bounding_box" | "screenshot_element"
        | "find_visual_text" | "find_element_at" | "compare_screenshots" | "visual_diff"
        | "assert_screenshot_matches" => super::visual::prepare(method, args),
        "screenshot" => super::extra::screenshot(args),
        _ => Err(format!("browser: no method `{}`", method)),
    }
}
