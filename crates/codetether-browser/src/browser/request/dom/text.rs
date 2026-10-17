//! Visible-text targeting and toggle request payloads.
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
/// Request payload for the click text operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::ClickTextRequest;
/// let request = ClickTextRequest { selector: None, frame_selector: None, text: "Submit".into(), timeout_ms: 1000, exact: true, index: 0 };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct ClickTextRequest {
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
    pub text: String,
    pub timeout_ms: u64,
    pub exact: bool,
    pub index: usize,
}
#[derive(Debug, Clone, Serialize)]
/// Request payload for the toggle operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::ToggleRequest;
/// let request = ToggleRequest { selector: "#agree".into(), frame_selector: None, text: "Agree".into(), timeout_ms: 1000 };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct ToggleRequest {
    pub selector: String,
    pub frame_selector: Option<String>,
    pub text: String,
    pub timeout_ms: u64,
}
