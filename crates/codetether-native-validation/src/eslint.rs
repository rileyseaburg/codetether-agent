//! Native access check and sandbox execution for shared ESLint validation.
use codetether_bash_tool::BashTool;
use codetether_tool_core::Tool;
use std::path::Path;

pub(super) async fn collect(workspace_dir: &Path, path: &Path) -> Vec<String> {
    let Some(args) = codetether_validation::eslint::request(workspace_dir, path) else {
        return Vec::new();
    };
    if codetether_native_runtime_policy::evaluate_tool_invocation("bash", &args)
        .await
        .is_some()
    {
        return Vec::new();
    }
    let Ok(result) = BashTool::new().execute(args).await else {
        return Vec::new();
    };
    codetether_validation::eslint::render(&result.output)
}
