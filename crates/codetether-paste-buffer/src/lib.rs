//! Large-text paste sidecars, placeholders and prompt expansion.
//!
//! Keep PendingTextPaste entries alongside the input buffer. Attach content
//! when should_summarize is true, then expand placeholders at submission.
//! Raw expansion reconstructs embedded data without delimiter banners.
mod attach;
mod entry;
mod expand;
mod raw;
mod size;
mod summarize;
pub use attach::attach_paste;
pub use entry::PendingTextPaste;
pub use expand::expand_paste_placeholders;
pub use raw::expand_placeholders_raw;
pub use size::format_size;
pub use summarize::should_summarize;
#[cfg(test)]
mod tests;
