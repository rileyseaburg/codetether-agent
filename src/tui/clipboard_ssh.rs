//! Compatibility exports for SSH and headless clipboard guidance.
pub use codetether_terminal_clipboard::{is_ssh_session, is_headless_session, is_ssh_or_headless, clipboard_unavailable_message};
#[cfg(test)]
#[path = "clipboard_ssh_tests.rs"]
mod tests;
