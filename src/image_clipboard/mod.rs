//! System clipboard access and portable image compatibility exports.
//!
//! System clipboard access is provided by the dedicated clipboard crate;
//! portable attachment helpers remain shared with providers.
pub use codetether_clipboard::{capture_image, copy_text};
pub use codetether_image::{ImageAttachment, attachment_from_data_url, fit, RejectReason, reject_reason};
