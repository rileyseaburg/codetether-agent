//! Fallback detection for terminals that replay a paste as individual keys.
//!
//! Pass a borrowed [View] to [enter_is_likely_paste_newline]. Detection retains
//! the terminal-specific slow first-line allowance and recent-key thresholds.
mod detect;
mod lines;
mod view;
pub use detect::{enter_is_likely, enter_is_likely_paste_newline};
pub use lines::current_line_len;
pub use view::View;
#[cfg(test)]
mod tests;
