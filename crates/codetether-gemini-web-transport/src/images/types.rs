use codetether_gemini_web_protocol::images::Continuation;

/// Downloaded, structurally validated generated image.
///
/// # Examples
/// ```
/// use codetether_gemini_web_transport::images::GeneratedImage;
/// let image = GeneratedImage {
///     bytes: vec![], mime_type: "image/png".into(), extension: "png".into(),
///     width: 1, height: 1,
/// };
/// assert_eq!(image.width, 1);
/// ```
pub struct GeneratedImage {
    /// Original image bytes; no base64 envelope.
    pub bytes: Vec<u8>,
    /// Validated MIME type: `image/jpeg` or `image/png`.
    pub mime_type: String,
    /// Safe extension without a leading dot: `jpg` or `png`.
    pub extension: String,
    /// Width extracted from the image header, in pixels.
    pub width: u32,
    /// Height extracted from the image header, in pixels.
    pub height: u32,
}

/// A generation's downloaded images and optional conversation continuation.
///
/// Keep continuation metadata private; supply it to the next
/// [`ImageClient::generate`](super::ImageClient::generate) to refine this image.
///
/// # Examples
/// ```
/// use codetether_gemini_web_transport::images::ImageGeneration;
/// let result = ImageGeneration { images: vec![], continuation: None };
/// assert!(result.continuation.is_none());
/// ```
pub struct ImageGeneration {
    /// All successfully downloaded image attachments.
    pub images: Vec<GeneratedImage>,
    /// Server identifiers for a follow-up in the same conversation, if supplied.
    pub continuation: Option<Continuation>,
}
