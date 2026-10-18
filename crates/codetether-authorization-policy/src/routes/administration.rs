//! administration route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/v1/k8s/scale",
        methods: Some(&["POST"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/k8s/restart",
        methods: Some(&["POST"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/k8s/",
        methods: Some(&["GET"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/k8s/subagent",
        methods: Some(&["POST", "DELETE"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/plugins",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/audit",
        methods: None,
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/audit/replay",
        methods: Some(&["GET"]),
        permission: "admin:access",
    },
];
