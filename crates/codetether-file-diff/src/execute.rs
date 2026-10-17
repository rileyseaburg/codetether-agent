//! execute for file comparisons.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let git_mode = args["git"].as_bool().unwrap_or(false);
    let staged = args["staged"].as_bool().unwrap_or(false);
    let context = args["context"].as_u64().unwrap_or(3);

    if git_mode {
        git::execute(&args, staged, context).await
    } else {
        files::execute(&args).await
    }
}
