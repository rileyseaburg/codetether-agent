//! Stable serialized names for audit categories and outcomes.

use crate::{AuditCategory, AuditOutcome};
use serde_json::json;

#[test]
fn enum_wire_names_are_stable() {
    for (category, name) in [
        (AuditCategory::Api, "api"),
        (AuditCategory::ToolExecution, "tool_execution"),
        (AuditCategory::Session, "session"),
        (AuditCategory::Cognition, "cognition"),
        (AuditCategory::Swarm, "swarm"),
        (AuditCategory::Auth, "auth"),
        (AuditCategory::K8s, "k8s"),
        (AuditCategory::Sandbox, "sandbox"),
        (AuditCategory::Config, "config"),
    ] {
        assert_eq!(serde_json::to_value(category).unwrap(), name);
        assert_eq!(
            serde_json::from_value::<AuditCategory>(json!(name)).unwrap(),
            category
        );
    }
    for (outcome, name) in [
        (AuditOutcome::Success, "success"),
        (AuditOutcome::Failure, "failure"),
        (AuditOutcome::Denied, "denied"),
    ] {
        assert_eq!(serde_json::to_value(outcome).unwrap(), name);
        assert_eq!(
            serde_json::from_value::<AuditOutcome>(json!(name)).unwrap(),
            outcome
        );
    }
}
