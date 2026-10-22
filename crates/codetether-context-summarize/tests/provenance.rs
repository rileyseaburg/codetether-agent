use codetether_context_summarize::ContextSummarizeTool;
use codetether_tool_core::Tool;
use serde_json::json;
mod support;

#[tokio::test]
async fn explicit_tool_returns_only_source_and_policy_valid_cache_hits() {
    let tool = ContextSummarizeTool::<support::Session>::cached_only();
    for id in ["valid", "edited", "legacy", "policy"] {
        let result = tool
            .execute(json!({"__ct_session_id": id, "start": 0, "end": 1}))
            .await
            .unwrap();
        assert_eq!(result.success, id == "valid", "{id}: {}", result.output);
        assert_eq!(result.output.contains("CACHED EVIDENCE"), id == "valid");
    }
}
