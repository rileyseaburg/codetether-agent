//! Shadow mode never silently routes an input action through physical injection.

use super::*;
use serde_json::json;

fn request(action: &str, mode: &str) -> ComputerUseInput {
    serde_json::from_value(json!({"action": action, "input_mode": mode, "hwnd": 123})).unwrap()
}

#[test]
fn unimplemented_app_selectors_cannot_redirect_ocr_or_shadow_input() {
    let mut input = request("ocr", "shadow");
    input.app = Some("not-an-enforced-selector".into());
    assert!(app_gated(&input));
    input.action = Action::Click;
    assert!(app_gated(&input));
    input.action = Action::OcrStatus;
    assert!(!app_gated(&input));
}

#[test]
fn shadow_input_actions_and_unsupported_focus_actions_stay_shadow() {
    for action in ["click", "right_click", "double_click", "drag", "mouse_down",
        "mouse_move", "mouse_up", "type_text", "set_text", "click_client", "press_key",
        "scroll", "bring_to_front", "focus_viewport", "blender_select_frame", "stop", "status"] {
        assert!(shadow(&request(action, "shadow")), "{action} must not become physical input");
        assert!(!shadow(&request(action, "physical")));
    }
}

#[test]
fn shadow_observation_actions_use_read_only_native_backends() {
    for action in ["ocr", "ocr_status", "snapshot", "window_snapshot", "list_apps", "wait_ms"] {
        assert!(!shadow(&request(action, "shadow")), "{action}");
    }
}

#[test]
fn physical_default_and_required_action_are_preserved() {
    let input: ComputerUseInput = serde_json::from_value(json!({"action":"ocr", "path":"screen.png", "language":"en-US"})).unwrap();
    assert_eq!(input.input_mode, InputMode::Physical);
    assert_eq!(input.ocr.path.unwrap(), std::path::Path::new("screen.png"));
    assert_eq!(input.ocr.language.as_deref(), Some("en-US"));
    assert!(serde_json::from_value::<ComputerUseInput>(json!({})).is_err());
    assert!(serde_json::from_value::<ComputerUseInput>(json!({"action":"click", "input_mode":"guess"})).is_err());
}