//! configuration route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/api/version",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/config/verifier-model",
        methods: Some(&["PUT", "DELETE"]),
        permission: "agent:write",
    },
    PolicyRule {
        pattern: "/api/config",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/provider",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/agent",
        methods: None,
        permission: "agent:read",
    },
];
