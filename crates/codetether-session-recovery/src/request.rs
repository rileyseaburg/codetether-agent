//! Metadata and optional prompt accepted by durable recovery.
use serde::Deserialize;

/// Metadata and optional prompt supplied by the dashboard resume flow.
/// # Examples
/// ```
/// let request = codetether_session_recovery::ResumeRequest { prompt: None, agent: None, model: None };
/// assert!(request.prompt.is_none());
/// ```
#[derive(Deserialize)]
pub struct ResumeRequest {
    pub prompt: Option<String>,
    pub agent: Option<String>,
    pub model: Option<String>,
}
