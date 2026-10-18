//! Clipboard image bridge command and presentation.
//!
//! [ClipboardArgs] selects the operation; [execute] uses the shared clipboard
//! backend to print an image data URL or place that text on the clipboard.
mod args;
mod execute;
mod image;
pub use args::{ClipboardArgs, ClipboardCommand, ClipboardImageArgs};
pub use execute::execute;
