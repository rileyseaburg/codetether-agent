use anyhow::{Result, anyhow};
use serde_json::Value;
use std::path::Path;

pub(super) fn normalize(args: &mut Value, root: &Path) -> Result<()> {
    let calls = args
        .get_mut("calls")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("batch calls must be an array"))?;
    for (index, call) in calls.iter_mut().enumerate() {
        let tool = call
            .get("tool")
            .or_else(|| call.get("name"))
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("batch call {index} is missing a tool name"))?
            .to_string();
        let nested = nested_args(call)
            .ok_or_else(|| anyhow!("batch call {index} is missing arguments"))?;
        super::normalize_tool_args(&tool, nested, root)
            .map_err(|error| anyhow!("batch call {index} ({tool}): {error}"))?;
    }
    Ok(())
}

fn nested_args(call: &mut Value) -> Option<&mut Value> {
    for key in ["args", "arguments", "params"] {
        if call.get(key).is_some() {
            return call.get_mut(key);
        }
    }
    None
}
