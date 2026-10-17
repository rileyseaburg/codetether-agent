//! Serializable non-text content accepted by child-agent messages.

use serde::{Deserialize, Serialize};

/// Image content retained in durable child mailboxes.
///
/// # Examples
/// ```
/// let image=codetether_collaboration_input::MessageImage {
///     data_url:"data:image/png;base64,AA==".into(), mime_type:Some("image/png".into()),
/// };
/// let attachment:codetether_image::ImageAttachment=image.into();
/// assert_eq!(attachment.mime_type.as_deref(),Some("image/png"));
/// ```
#[derive(Clone, Deserialize, Serialize)]
pub struct MessageImage {
    /// Image data URL or explicit HTTP(S) reference forwarded to the provider.
    pub data_url: String,
    /// MIME type parsed locally; absent for references the provider will fetch.
    pub mime_type: Option<String>,
}

impl From<MessageImage> for codetether_image::ImageAttachment {
    fn from(image: MessageImage) -> Self {
        Self {
            data_url: image.data_url,
            mime_type: image.mime_type,
        }
    }
}
