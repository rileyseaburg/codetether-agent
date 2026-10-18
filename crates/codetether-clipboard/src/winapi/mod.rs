//! Windows clipboard access with portable no-operation stubs on other systems.
//!
//! [get_clipboard_text], [set_clipboard_text] and [get_clipboard_image] preserve
//! the optional-result contract used by terminal clipboard fallbacks.
mod dib;
mod dib_bytes;
mod dib_layout;
#[cfg(windows)]
mod guard;
#[cfg(windows)]
mod image;
#[cfg(windows)]
mod image_encode;
#[cfg(windows)]
mod image_read;
#[cfg(windows)]
mod text_read;
#[cfg(windows)]
mod text_write;
#[cfg(test)]
mod windows_dib_tests;
#[cfg(windows)]
pub use image::get_clipboard_image;
#[cfg(windows)]
pub use text_read::get_clipboard_text;
#[cfg(windows)]
pub use text_write::set_clipboard_text;

#[cfg(not(windows))]
/// Read plain text from the Windows clipboard backend.
/// # Arguments
/// None; reads the current process's system clipboard.
/// # Returns
/// Nonempty text, or None when unavailable.
/// # Examples
/// ```no_run
/// let _text = codetether_clipboard::winapi::get_clipboard_text();
/// ```
pub fn get_clipboard_text() -> Option<String> {
    None
}

#[cfg(not(windows))]
/// Write plain text to the Windows clipboard backend.
/// # Arguments
/// * `text` - Text to transfer to the system clipboard.
/// # Returns
/// Some unit on success, or None when unavailable.
/// # Examples
/// ```no_run
/// let _result = codetether_clipboard::winapi::set_clipboard_text("hello");
/// ```
pub fn set_clipboard_text(_text: &str) -> Option<()> {
    None
}

#[cfg(not(windows))]
/// Read an image from the Windows clipboard backend.
/// # Arguments
/// None; reads the current process's system clipboard.
/// # Returns
/// A PNG attachment, or None when unavailable or undecodable.
/// # Examples
/// ```no_run
/// let _image = codetether_clipboard::winapi::get_clipboard_image();
/// ```
pub fn get_clipboard_image() -> Option<codetether_image::ImageAttachment> {
    None
}
