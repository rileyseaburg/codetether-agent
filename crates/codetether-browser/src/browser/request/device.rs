use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PointerClick {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyboardTypeRequest {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyboardPressRequest {
    pub key: String,
}
