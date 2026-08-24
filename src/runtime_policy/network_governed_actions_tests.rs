//! Mixed-action network classification tests.

use serde_json::json;

#[test]
fn mux_list_is_network_governed() {
    assert_eq!(
        super::check("mux_control", &json!({"action": "list"})),
        Some(true)
    );
}

#[test]
fn every_mixed_capability_action_is_classified() {
    for action in ["init", "delegate", "handoff", "complete"] {
        assert_eq!(
            super::check("relay_autochat", &json!({"action":action})),
            Some(true)
        );
    }
    for action in ["status", "unknown"] {
        assert_eq!(
            super::check("relay_autochat", &json!({"action":action})),
            Some(false)
        );
    }
    assert_eq!(super::check("go", &json!({"action":"execute"})), Some(true));
    assert_eq!(super::check("go", &json!({"action":"status"})), Some(false));
    assert_eq!(super::check("ralph", &json!({"action":"run"})), Some(true));
    assert_eq!(
        super::check("ralph", &json!({"action":"status"})),
        Some(false)
    );
    assert_eq!(
        super::check("session_recall", &json!({"mode":"answer"})),
        Some(true)
    );
    assert_eq!(
        super::check("session_recall", &json!({"mode":"evidence"})),
        Some(false)
    );
    for action in ["list", "read", "start", "stop", "unknown"] {
        assert_eq!(
            super::check("mux_control", &json!({"action":action})),
            Some(true)
        );
    }
}
