use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EvalRequest {
    pub source: String,
    pub frame_selector: Option<String>,
}
