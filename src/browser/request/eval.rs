use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EvalRequest {
    pub expression: String,
}
