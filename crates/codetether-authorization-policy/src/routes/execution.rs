//! execution route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/v1/tasks/dispatch",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/voice/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/voice/",
        methods: Some(&["POST", "DELETE"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/agent/codebases/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/cognition/proposals/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
];
