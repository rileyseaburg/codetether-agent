//! Selector and optional scope request payloads.
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
/// Request payload for the selector operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::SelectorRequest;
/// let request = SelectorRequest { selector: "#item".into(), frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct SelectorRequest {
    pub selector: String,
    pub frame_selector: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
/// Request payload for the scope operation.
///
/// # Examples
/// ```
/// use codetether_browser::browser::request::ScopeRequest;
/// let request = ScopeRequest { selector: None, frame_selector: None };
/// assert!(serde_json::to_value(request).unwrap().is_object());
/// ```
pub struct ScopeRequest {
    pub selector: Option<String>,
    pub frame_selector: Option<String>,
}
