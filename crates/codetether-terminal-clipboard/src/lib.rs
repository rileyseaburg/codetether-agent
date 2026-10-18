//! Terminal clipboard reads, copy fallback and environment guidance.
//!
//! [get_clipboard_image] and [get_clipboard_text] probe the actual clipboard.
//! [copy_text] first writes the system clipboard, then tries terminal OSC52.
//! Environment hints guide users without disabling forwarded clipboard access.
mod access;
mod environment;
mod guidance;
mod osc52;
mod platform;
mod text;
pub use access::{get_clipboard_image, get_clipboard_text, is_ssh_or_headless};
pub use environment::{is_headless_session, is_ssh_session};
pub use guidance::clipboard_unavailable_message;
pub use text::copy_text;
