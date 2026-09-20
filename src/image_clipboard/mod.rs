//! Clipboard image bridge helpers.
//!
//! These helpers support SSH sessions by converting a local clipboard image
//! into a text data URL that can be pasted through a terminal.

mod capture;
mod data_url;
mod reject;

#[cfg(test)]
mod tests;

pub use capture::{capture_image, copy_text};
pub use data_url::attachment_from_data_url;
pub use reject::{RejectReason, reject_reason};
