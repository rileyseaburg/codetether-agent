//! Every transcript turn must be represented before continuation synthesis.
#[path = "continuation_fixture.rs"]
mod fixture;
#[path = "../router/consolidation/provider.rs"]
mod provider;
use crate::router::CrateAutoProcessContext;
use std::sync::Arc;

#[tokio::test]
async fn continuation_keeps_chronological_evidence_despite_large_tool_output() {
    let source = fixture::source();
    let provider = Arc::new(provider::Provider::new(vec![provider::text(
        "Retained evidence",
    )]));
    let context = CrateAutoProcessContext {
        tool_id: "summary_index",
        tool_args: serde_json::json!({"target_tokens":768}),
        session_id: "coverage",
        abort: None,
        on_progress: None,
        provider: provider.clone(),
        model: "mock".into(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
        rewriter: None,
    };
    super::process(&source, &context, &Default::default())
        .await
        .unwrap();
    let calls = provider.calls.lock().unwrap();
    let prompt = calls[0]["messages"][1]["text"].as_str().unwrap();
    for index in 1..15 {
        assert!(
            prompt.contains(&format!("[ToolResult id=call-{index}]")),
            "missing call {index}"
        );
        assert!(
            prompt.contains(&format!("Observed result {index}:")),
            "missing outcome {index}"
        );
    }
    assert!(prompt.contains("Use the current local server"));
    assert!(prompt.contains("Test through the funnel"));
    assert!(prompt.len() < 70_000);
}
