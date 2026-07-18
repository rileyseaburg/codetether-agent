//! Serializable non-text content accepted by child-agent messages.

use serde::{Deserialize, Serialize};

/// Image content retained in durable child mailboxes.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct MessageImage {
    /// Base64-encoded image data URL.
    pub(crate) data_url: String,
    /// MIME type parsed from the data URL or local filename.
    pub(crate) mime_type: Option<String>,
}

impl From<MessageImage> for crate::session::ImageAttachment {
    fn from(image: MessageImage) -> Self {
        Self {
            data_url: image.data_url,
            mime_type: image.mime_type,
        }
    }
}
