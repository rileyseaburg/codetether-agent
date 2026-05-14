use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ScrollRequest {
    pub selector: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub frame_selector: Option<String>,
}
