use super::Continuation;

/// Structured image references and optional state for a follow-up request.
///
/// URLs are raw, untrusted server strings. They may contain signed access
/// tokens: do not log them. Transport must validate destinations and choose
/// renditions before fetching. A missing continuation means metadata was
/// absent, invalid, or associated with multiple image candidates.
///
/// # Examples
/// ```
/// use codetether_gemini_web_protocol::images::ImageResponse;
/// let result = ImageResponse { urls: vec!["https://example.test/image".into()],
///                              continuation: None };
/// assert_eq!(result.urls.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageResponse {
    /// Deduplicated structured image references in first-seen order.
    pub urls: Vec<String>,
    /// Valid state belonging to the sole image-bearing candidate, if available.
    pub continuation: Option<Continuation>,
}
