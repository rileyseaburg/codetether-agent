//! Portable image attachment data.

/// An image attachment to include with a user message (e.g. pasted from the
/// clipboard in the TUI).
///
/// # Examples
///
/// ```rust
/// use codetether_image::ImageAttachment;
///
/// let img = ImageAttachment {
///     data_url: "data:image/png;base64,iVBORw0KGgo...".to_string(),
///     mime_type: Some("image/png".to_string()),
/// };
/// assert!(img.data_url.starts_with("data:image/png;base64"));
/// assert_eq!(img.mime_type.as_deref(), Some("image/png"));
/// ```
#[derive(Debug, Clone)]
pub struct ImageAttachment {
    /// Base64-encoded data URL, e.g. `"data:image/png;base64,iVBOR..."`.
    pub data_url: String,
    /// MIME type of the image, e.g. `"image/png"`.
    pub mime_type: Option<String>,
}
