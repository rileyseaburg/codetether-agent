//! task workers route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/v1/agent/tasks",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/agent/tasks",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/agent/tasks/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/worker/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/worker/tasks/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/agent/workers",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
];
