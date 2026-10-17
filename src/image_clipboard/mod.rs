//! System clipboard access and portable image compatibility exports.
//!
//! OS capture remains here; attachment parsing, fitting, and rejection handling
//! are shared with providers through the portable image crate.
mod capture;
pub use capture::{capture_image, copy_text};
pub use codetether_image::{ImageAttachment, attachment_from_data_url, fit, RejectReason, reject_reason};
