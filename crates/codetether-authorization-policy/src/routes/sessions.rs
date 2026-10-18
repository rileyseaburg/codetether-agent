//! sessions route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/api/session/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/api/session",
        methods: Some(&["POST"]),
        permission: "sessions:write",
    },
    PolicyRule {
        pattern: "/api/session",
        methods: Some(&["GET"]),
        permission: "sessions:read",
    },
    PolicyRule {
        pattern: "/api/realtime/session/",
        methods: Some(&["GET"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/mcp/v1/tools",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
];
