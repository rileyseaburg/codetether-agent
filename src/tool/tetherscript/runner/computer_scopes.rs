/// Return every scope understood by the v1 computer capability.
pub fn all() -> Vec<String> {
    [
        "computer.inspect",
        "computer.window",
        "computer.pointer",
        "computer.keyboard",
        "computer.wait",
        "computer.session",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

/// Map a computer-use action to its required scope.
pub fn for_action(action: &str) -> Option<&'static str> {
    match action {
        "status" | "list_apps" | "snapshot" | "window_snapshot" => Some("computer.inspect"),
        "request_app" | "bring_to_front" | "focus_viewport" => Some("computer.window"),
        "click" | "right_click" | "double_click" | "drag" => Some("computer.pointer"),
        "mouse_down" | "mouse_move" | "mouse_up" | "scroll" => Some("computer.pointer"),
        "type_text" | "press_key" | "blender_select_frame" => Some("computer.keyboard"),
        "wait_ms" => Some("computer.wait"),
        "stop" => Some("computer.session"),
        _ => None,
    }
}
