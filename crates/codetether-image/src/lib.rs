//! Portable image attachments, data-URL validation, and size fitting.
//!
//! [`ImageAttachment`] is shared between user interfaces and provider adapters.
//! Use [`attachment_from_data_url`] for pasted text, [`attachment_from_rgba`]
//! for captured pixels, and [`fit::fit`] to fit an encoded image. Clipboard
//! access and conversation persistence belong to application adapters.
mod attachment;
mod data_url;
pub mod fit;
mod reject;
mod rgba;
pub use attachment::ImageAttachment;
pub use data_url::attachment_from_data_url;
pub use reject::{RejectReason,reject_reason};
pub use rgba::attachment_from_rgba;
#[cfg(test)]
mod tests;
