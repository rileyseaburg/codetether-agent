//! tools route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/v1/tools",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/tools/register",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/tools/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/a2a/",
        methods: None,
        permission: "",
    },
];
