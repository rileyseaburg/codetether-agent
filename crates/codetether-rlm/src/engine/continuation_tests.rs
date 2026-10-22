//! Verify the real synthesis request retains continuation-specific instructions.
#[path = "../router/consolidation/provider.rs"]
mod provider;
use crate::router::CrateAutoProcessContext;
use std::sync::Arc;

#[tokio::test]
async fn continuation_synthesis_preserves_provenance_query_and_budget() {
    let source = "[0 User]\nUse the production CLI and check the database.\n\n---\n\n\
        [1 Assistant]\n[Thinking]\nI infer that I need to build and push first.\n\n---\n\n\
        [2 User]\nI never told you to push anything.\n\n---\n\n";
    for tool_id in [
        "proactive_session_context",
        "summary_index",
        "session_context",
        "context_reset",
    ] {
        let provider = Arc::new(provider::Provider::new(vec![provider::text(
            "User requests CLI/database checks and explicitly rejects pushing. Building was an assistant inference.",
        )]));
        let context = CrateAutoProcessContext {
            tool_id,
            tool_args: serde_json::json!({"target_tokens": 512}),
            session_id: "continuation-provenance-fixture",
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
        let result = super::process(source, &context, &Default::default())
            .await
            .unwrap()
            .unwrap();
        assert!(result.processed.contains("assistant inference"));
        let calls = provider.calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        let prompt = calls[0]["messages"][1]["text"].as_str().unwrap();
        let (question, evidence) = prompt.split_once("Evidence records selected:").unwrap();
        assert!(question.contains("PRIMARY GOAL"), "{tool_id}: {question}");
        assert!(question.contains("Preserve provenance"));
        assert!(question.contains("latest user corrections"));
        assert!(question.contains("exact callable contracts"));
        assert!(question.contains("within 512 tokens"));
        assert!(evidence.contains("I never told you to push anything"));
        assert!(evidence.contains("I infer that I need to build and push first"));
    }
}
