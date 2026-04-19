use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SelectorRequest {
    pub selector: String,
    pub frame_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FillRequest {
    pub selector: String,
    pub value: String,
    pub frame_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeRequest {
    pub selector: String,
    pub text: String,
    pub delay_ms: u64,
    pub frame_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyPressRequest {
    pub selector: String,
    pub key: String,
    pub frame_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScopeRequest {
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClickTextRequest {
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
    pub text: String,
    pub timeout_ms: u64,
    pub exact: bool,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToggleRequest {
    pub selector: String,
    pub frame_selector: Option<String>,
    pub text: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadRequest {
    pub selector: String,
    pub paths: Vec<String>,
    pub frame_selector: Option<String>,
}
