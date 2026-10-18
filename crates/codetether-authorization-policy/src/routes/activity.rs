//! activity route rules in original precedence order.
use super::PolicyRule;
pub(super) const RULES: &[PolicyRule] = &[
    PolicyRule {
        pattern: "/v1/cognition/start",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/cognition/stop",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/cognition/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/swarm/personas",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/swarm/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/swarm/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/bus/stream",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/bus/publish",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
];
