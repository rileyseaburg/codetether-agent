//! Auto-process context construction for executor bridge.

use serde_json::json;
use std::sync::Arc;

use super::RlmExecutor;
use crate::rlm::router::AutoProcessContext;

pub(super) fn auto_context(exec: &RlmExecutor, query: &str) -> AutoProcessContext<'static> {
    AutoProcessContext {
        tool_id: "rlm_executor",
        tool_args: json!({ "query": query }),
        session_id: "rlm-executor",
        abort: None,
        on_progress: None,
        provider: Arc::clone(&exec.provider),
        model: exec.model.clone(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    }
}
