//! Mux delegation-boundary tests.

use serde_json::json;

#[test]
fn work_cannot_be_handed_to_another_agent() {
    for action in ["ask", "spawn"] {
        let input = json!({ "action": action });
        assert_eq!(
            super::delegation::work_action("agent", &input),
            Some(action)
        );
    }
    for action in ["list", "message", "status", "interrupt", "close"] {
        let input = json!({ "action": action });
        assert!(super::delegation::work_action("agent", &input).is_none());
    }
}

#[test]
fn rejection_tells_the_model_to_continue_locally() {
    let (output, success, metadata) = super::gate_error::delegation("agent", "spawn");
    assert!(!success);
    assert!(output.contains("cannot create or delegate work"));
    assert!(output.contains("Continue locally"));
    assert!(output.contains("Steering an existing mux session"));
    assert_eq!(
        metadata.unwrap()["error_code"],
        "MUX_AGENT_DELEGATION_FORBIDDEN"
    );
}
