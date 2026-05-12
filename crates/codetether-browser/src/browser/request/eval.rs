use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EvalRequest {
    pub expression: String,
    pub timeout_ms: u64,
}
