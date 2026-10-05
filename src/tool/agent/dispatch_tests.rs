//! Regression coverage for removed mux-backed agent actions.

use super::{Params, execute};
use serde_json::json;

#[tokio::test]
async fn mux_tui_actions_are_rejected_even_with_a_target() {
    for action in ["read", "interact"] {
        let params: Params = serde_json::from_value(json!({
            "action": action,
            "name": "independent-mux-session"
        }))
        .unwrap();
        let result = execute(&params).await.unwrap();
        assert!(!result.success, "{action} must not reach a mux TUI");
        assert!(result.output.contains(action));
        let allowed = result.output.split("Valid:").nth(1).unwrap();
        assert!(!allowed.contains("read"));
        assert!(!allowed.contains("interact"));
    }
}

#[tokio::test]
async fn removed_mux_actions_do_not_require_a_target() {
    for action in ["read", "interact"] {
        let params: Params = serde_json::from_value(json!({"action": action})).unwrap();
        let result = execute(&params)
            .await
            .expect("removed actions should be rejected before resolving a target");
        assert!(!result.success);
        assert!(result.output.contains(action));
    }
}
