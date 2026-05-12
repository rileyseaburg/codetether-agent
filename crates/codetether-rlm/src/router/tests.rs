//! Router unit tests.

use crate::capability::output_capability;
use crate::config::RlmConfig;
use crate::router::{RoutingContext, should_route};
use crate::router::extract::extract_final;

#[test]
fn webfetch_routes_when_oversized() {
    let output = "a".repeat(200_000);
    let ctx = RoutingContext {
        tool_id: "webfetch".into(), session_id: "s".into(), call_id: None,
        model_context_limit: 200_000, current_context_tokens: Some(1000),
    };
    assert!(should_route(&output, &ctx, &RlmConfig::default()).should_route);
}

#[test]
fn unknown_tools_never_route() {
    let output = "a".repeat(200_000);
    let ctx = RoutingContext {
        tool_id: "some_new_tool".into(), session_id: "s".into(), call_id: None,
        model_context_limit: 200_000, current_context_tokens: Some(1000),
    };
    let r = should_route(&output, &ctx, &RlmConfig::default());
    assert!(!r.should_route);
    assert_eq!(r.reason, "tool_unknown_capability_no_route");
}

#[test]
fn exact_content_tools_never_route() {
    let output = "a".repeat(500_000);
    let ctx = RoutingContext {
        tool_id: "read".into(), session_id: "s".into(), call_id: None,
        model_context_limit: 200_000, current_context_tokens: Some(1000),
    };
    let r = should_route(&output, &ctx, &RlmConfig::default());
    assert!(!r.should_route);
    assert_eq!(r.reason, "tool_exact_content_no_route");
}

#[test]
fn extract_final_works() {
    assert_eq!(extract_final(r#"FINAL("hello")"#), Some("hello".into()));
    assert_eq!(extract_final(r#"FINAL('') x"#), None);
    assert_eq!(extract_final("no final"), None);
}
