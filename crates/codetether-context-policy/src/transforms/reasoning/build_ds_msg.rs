//! build ds msg conversion.
use super::*;

/// Build a DeepSeek-API-shaped JSON object for the tetherscript hook.
pub fn build_ds_msg(msg: &Message) -> Value {
    let mut ds = serde_json::Map::new();
    ds.insert("role".into(), role_str(msg.role).into());
    ds.insert("content".into(), collect_text(msg).into());
    ds.insert("reasoning_content".into(), collect_reasoning(msg));
    ds.insert("tool_calls".into(), collect_tool_calls(msg));
    Value::Object(ds)
}
