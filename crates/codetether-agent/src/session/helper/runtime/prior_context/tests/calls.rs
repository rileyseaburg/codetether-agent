use super::user;
use crate::session::helper::runtime::{
    block_prior_context, block_serialized_prior_context, prior_context_tool_available,
    remove_prior_context_tools,
};
use crate::tool::{ToolRegistry, context_browse::ContextBrowseTool};
use serde_json::json;
use std::sync::Arc;

fn denied() -> Vec<crate::provider::Message> {
    vec![user("do not inspect session history")]
}

#[test]
fn blocks_direct_history_tools() {
    for tool in ["session_recall", "context_browse", "memory", "search"] {
        let result = block_prior_context(&denied(), tool, &json!({})).expect("blocked");
        assert_eq!(
            result.metadata["error_code"],
            "PRIOR_CONTEXT_DISABLED_BY_USER"
        );
        assert!(!prior_context_tool_available(&denied(), tool));
    }
    assert!(block_prior_context(&denied(), "read", &json!({})).is_none());
}

#[test]
fn removes_history_tools_from_registry() {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(ContextBrowseTool));
    remove_prior_context_tools(&mut registry);
    assert!(registry.get("context_browse").is_none());
}

#[test]
fn blocks_history_tools_nested_in_batch() {
    let args = json!({"calls": [{"tool": "session_recall", "args": {"query": "scope"}}]});
    assert!(block_prior_context(&denied(), "batch", &args).is_some());
    assert!(block_serialized_prior_context(&denied(), "batch", &args.to_string()).is_some());

    let aliased = json!({"calls": [{"name": "batch", "params": {
        "calls": [{"name": "session_recall", "params": {}}]
    }}]});
    assert!(block_prior_context(&denied(), "batch", &aliased).is_some());
}
