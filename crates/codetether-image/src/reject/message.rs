//! User-facing descriptions of rejected attachments.
use super::RejectReason;
use crate::data_url::MAX_IMAGE_DECODED_BYTES;
impl RejectReason {
    /// A user-facing explanation with the concrete limit.
    pub fn message(self) -> String {
        let mb = MAX_IMAGE_DECODED_BYTES / (1024 * 1024);
        match self {
            Self::TooLarge => {
                format!(
                    "Image too large to attach (limit {mb} MB). Resize it or use /image <path>."
                )
            }
            Self::UnsupportedMime => {
                "Unsupported image type. Use png, jpeg, gif, webp, bmp, or svg.".to_string()
            }
            Self::MalformedBase64 => {
                "Pasted image data was incomplete or corrupted; paste it again.".to_string()
            }
        }
    }
}
