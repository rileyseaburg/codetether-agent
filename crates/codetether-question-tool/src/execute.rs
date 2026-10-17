//! execute for the question tool.
use super::*;

pub(super) async fn execute(params: Value) -> Result<ToolResult> {
    let request = match Request::parse(params.clone()) {
        Ok(request) => request,
        Err(error) => return Ok(ToolResult::error(error.to_string())),
    };
    let session = params.get("__ct_session_id").and_then(Value::as_str);
    match runtime::ask(request, session).await {
        Ok(answers) => Ok(ToolResult::success(json!({"answers": answers}).to_string())
            .with_metadata("answers", json!(answers))),
        Err(error) => Ok(ToolResult::error(error.to_string())),
    }
}
