//! Frontend composer selection for pasted image URL ingestion.

#[cfg(test)]
#[path = "image_data_paste_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "image_data_reject_tests.rs"]
mod reject_tests;

codetether_image::declare_frontend_image_paste!(crate::tui::app::state::App);

#[cfg(test)]
include!("image_data_paste_test_imports.rs");
