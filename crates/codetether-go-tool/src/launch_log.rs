//! Log the start of an autonomous pipeline with its established target.
use uuid::Uuid;
pub(crate) fn starting(task: &str, okr_id: Uuid, resolved_model: &str, max_iterations: usize) {
    tracing::info!(target: "codetether_agent::tool::go",
        task = %task,
        okr_id = %okr_id,
        model = %resolved_model,
        max_iterations,
        "Starting /go autonomous pipeline via MCP (background)"
    );
}
