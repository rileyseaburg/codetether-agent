//! Linux advertises observation without silently accepting Windows-only controls.

use super::*;

#[test]
fn headless_status_reports_capabilities_without_claiming_capture_readiness() {
    let result = status::result(Session::Headless);
    assert!(result.success);
    let status: serde_json::Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(status["platform"], "linux");
    assert_eq!(status["session_type"], "headless");
    assert_eq!(status["screen_capture_ready"], false);
    assert_eq!(
        status["supported_actions"],
        serde_json::json!(["status", "snapshot"])
    );
}

#[tokio::test]
async fn unsupported_actions_and_window_selectors_fail_explicitly() {
    for action in ["click", "ocr", "list_apps", "window_snapshot", "stop"] {
        let result = super::super::dispatch(&request(action)).await.unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Windows-only"));
    }
    for field in [
        "hwnd",
        "app",
        "window_title_contains",
        "client_area",
        "viewport_child_hwnd",
    ] {
        let mut input = serde_json::json!({"action": "snapshot"});
        input[field] = match field {
            "app" | "window_title_contains" => serde_json::json!("test"),
            "client_area" => serde_json::json!(true),
            _ => serde_json::json!(1),
        };
        let result = super::super::dispatch(&serde_json::from_value(input).unwrap())
            .await
            .unwrap();
        assert!(!result.success && result.output.contains("selectors are unsupported"));
    }
}
