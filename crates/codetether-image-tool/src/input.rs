//! input for image loading.
use super::*;

/// Input parameters for the image tool.
///
/// # Examples
/// ```rust
/// let input = codetether_image_tool::ImageToolInput { path: "image.png".into(), detail: None };
/// assert!(input.detail.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageToolInput {
    /// Path to the image file or URL
    pub path: String,
    /// Optional detail level (low, high, auto) - for providers that support it
    #[serde(default)]
    pub detail: Option<String>,
}
