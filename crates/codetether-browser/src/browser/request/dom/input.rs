//! Editable input, keyboard, and file-upload payloads.
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
/// Request payload for the fill operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::FillRequest;
/// let request = FillRequest { selector: "#name".into(), value: "Alice".into(), frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct FillRequest {
    pub selector: String,
    pub value: String,
    pub frame_selector: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
/// Request payload for the type operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::TypeRequest;
/// let request = TypeRequest { selector: "#name".into(), text: "Alice".into(), delay_ms: 0, frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct TypeRequest {
    pub selector: String,
    pub text: String,
    pub delay_ms: u64,
    pub frame_selector: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
/// Request payload for the key press operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::KeyPressRequest;
/// let request = KeyPressRequest { selector: "#name".into(), key: "Enter".into(), frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct KeyPressRequest {
    pub selector: String,
    pub key: String,
    pub frame_selector: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
/// Request payload for the upload operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::UploadRequest;
/// let request = UploadRequest { selector: "#file".into(), paths: vec!["file.txt".into()], frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct UploadRequest {
    pub selector: String,
    pub paths: Vec<String>,
    pub frame_selector: Option<String>,
}
