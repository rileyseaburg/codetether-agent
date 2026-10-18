//! Frontend composer selection for pasted image URL ingestion.
use codetether_image as extract;
pub(crate) use extract::extract_image_data_url;

#[cfg(test)]
#[path = "image_data_paste_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "image_data_reject_tests.rs"]
mod reject_tests;

codetether_image::declare_frontend_image_paste!(crate::tui::app::state::App);
