//! Existing project-memory load and availability reporting.
use std::path::Path;
/// Load project beliefs and report when they produce nonempty context.
/// # Arguments
/// * `workspace_dir` - Workspace whose project memory is inspected.
/// # Returns
/// Unit after memory lookup and optional tracing.
/// # Examples
/// ```no_run
/// codetether_run_context::load_memory(std::path::Path::new("."));
/// ```
pub fn load_memory(workspace_dir: &Path) {
    let beliefs = codetether_cognition_core::palace::load_project_beliefs(workspace_dir);
    if !beliefs.is_empty() {
        let ctx = codetether_cognition_core::palace::belief_context(&beliefs);
        if !ctx.is_empty() {
            tracing::info!(beliefs = beliefs.len(), "Loaded project memory palace");
        }
    }
}
