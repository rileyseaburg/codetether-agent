use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EvalOutput {
    pub result: serde_json::Value,
}
