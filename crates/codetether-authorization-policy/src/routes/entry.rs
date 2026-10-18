//! entry route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/health",
        methods: None,
        permission: "",
    },
    PolicyRule {
        pattern: "/task",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/knative/tasks/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/knative/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/models",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/models",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/chat/completions",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
];
