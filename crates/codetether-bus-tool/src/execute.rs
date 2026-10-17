//! Query and format bounded bus history.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let limit = args
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(20)
        .min(200) as usize;
    let prefix = args.get("topic_prefix").and_then(Value::as_str);

    let Some(bus) = codetether_bus_core::global::global() else {
        return Ok(ToolResult::error(
            "No global agent bus is installed in this process.",
        ));
    };

    let envelopes = bus.recorder.recent(limit, prefix);
    let count = envelopes.len();
    let body = json!({ "count": count, "envelopes": envelopes });
    Ok(ToolResult::success(serde_json::to_string_pretty(&body)?)
        .with_metadata("count", json!(count)))
}
