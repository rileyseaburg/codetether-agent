//! glob search for core file tools.
use super::*;
pub(super) async fn execute(tool: &GlobTool, args: Value) -> Result<ToolResult> {
    let pattern = match args["pattern"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "glob",
                "pattern is required",
                Some(vec!["pattern"]),
                Some(json!({"pattern": "src/**/*.rs"})),
            ));
        }
    };
    let limit = args["limit"].as_u64().unwrap_or(100) as usize;

    let mut matches = Vec::new();

    let (base_dir, match_pattern) = glob_pattern::resolve(&tool.root, pattern);
    let compiled = glob::Pattern::new(&match_pattern)?;

    glob_walk::collect(&tool.root, &base_dir, &compiled, limit, &mut matches);
    let truncated = matches.len() >= limit;
    let output = matches.join("\n");

    Ok(ToolResult::success(output)
        .with_metadata("count", json!(matches.len()))
        .with_metadata("truncated", json!(truncated)))
}
