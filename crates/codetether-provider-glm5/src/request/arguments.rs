//! vLLM function argument JSON normalization.
use crate::*;
pub(super) fn encode(arguments: &str) -> String {
    let args_string = serde_json::from_str::<Value>(arguments)
        .map(|parsed| serde_json::to_string(&parsed).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|_| json!({"input": arguments}).to_string());
    args_string
}
