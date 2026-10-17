//! Capability and context-pressure routing cases.
use crate::{RlmConfig, RoutingContext};
pub(super) fn capture() {
    let mut routes = Vec::new();
    for mode in ["off", "auto", "always"] {
        for tool in ["webfetch", "read", "unknown"] {
            let config = RlmConfig {
                mode: mode.into(),
                threshold: 0.35,
                ..Default::default()
            };
            for current in [None, Some(900)] {
                let ctx = RoutingContext {
                    tool_id: tool.into(),
                    session_id: "fixture".into(),
                    call_id: None,
                    model_context_limit: 1000,
                    current_context_tokens: current,
                };
                routes.push(crate::should_route(&"a".repeat(500), &ctx, &config));
            }
        }
    }
    println!("ROUTES={}", serde_json::to_string(&routes).unwrap());
}
