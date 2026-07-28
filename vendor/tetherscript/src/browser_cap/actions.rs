//! browserctl action classification for scopes and approval.

pub(crate) fn scope_for_action(action: &str) -> Option<&'static str> {
    match action {
        "goto" | "back" | "reload" | "tabs" | "tabs_select" | "tabs_new" | "tabs_close"
        | "start" | "stop" => Some("browser.navigate"),
        "click" | "upload" | "fill" | "type" | "press" | "hover" | "focus" | "blur" | "scroll"
        | "click_text" | "fill_native" | "toggle" | "mouse_click" | "keyboard_type"
        | "keyboard_press" => Some("browser.interact"),
        "snapshot" | "text" | "html" | "wait" | "eval" => Some("browser.inspect.dom"),
        "network_log" | "diagnose" => Some("browser.inspect.network"),
        "fetch" | "axios" | "xhr" | "replay" => Some("browser.replay.network"),
        "screenshot" => Some("browser.screenshot"),
        "health" | "detect" => Some("browser.inspect.dom"),
        _ => None,
    }
}

pub(crate) fn is_mutating(action: &str) -> bool {
    matches!(
        action,
        "goto"
            | "back"
            | "reload"
            | "tabs_select"
            | "tabs_new"
            | "tabs_close"
            | "start"
            | "stop"
            | "click"
            | "upload"
            | "fill"
            | "type"
            | "press"
            | "hover"
            | "focus"
            | "blur"
            | "scroll"
            | "click_text"
            | "fill_native"
            | "toggle"
            | "mouse_click"
            | "keyboard_type"
            | "keyboard_press"
            | "fetch"
            | "axios"
            | "xhr"
            | "replay"
    )
}
