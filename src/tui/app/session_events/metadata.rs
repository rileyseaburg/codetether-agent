//! Surface structured tool metadata instead of silently dropping it.

use crate::tui::app::state::App;
use serde_json::Value;

pub(super) fn show(app: &mut App, name: &str, metadata: &Value) {
    app.state.status = if let Some(decision) = metadata.get("approval_decision") {
        let id = decision
            .get("request_id")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let status = decision
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("recorded");
        format!("Approval `{id}` {status}")
    } else {
        format!("{name} metadata updated")
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn approval_decision_is_visible() {
        let mut app = crate::tui::app::state::App::default();
        super::show(
            &mut app,
            "bash",
            &serde_json::json!({
                "approval_decision": {"request_id": "approval-1", "status": "approved"}
            }),
        );
        assert_eq!(app.state.status, "Approval `approval-1` approved");
    }
}
