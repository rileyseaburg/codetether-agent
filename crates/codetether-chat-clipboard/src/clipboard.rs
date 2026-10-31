//! Native clipboard paste: images first on Windows, text first elsewhere.
use crate::{
    Host,
    selection::{self, Content},
};

/// Paste a native clipboard image or text into the chat composer.
///
/// Windows images take precedence over accompanying labels or URLs. Plain text
/// still uses the shared paste handler so newlines do not submit separate turns.
/// Other platforms retain their text-first policy.
///
/// # Arguments
/// * `host` — Chat state and frontend effects.
/// # Returns
/// Updates the composer and status; unavailable clipboard data produces guidance.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_chat_clipboard::Host) {
/// codetether_chat_clipboard::handle_clipboard_paste(host);
/// # }
/// ```
pub fn handle_clipboard_paste(host: &mut impl Host) {
    match selection::read(
        cfg!(windows),
        codetether_terminal_clipboard::get_clipboard_text,
        codetether_terminal_clipboard::get_clipboard_image,
    ) {
        Some(Content::Text(text)) => {
            let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
            host.paste_into_chat(&normalized);
        }
        Some(Content::Image(image)) => {
            host.pending_images().push(image);
            let n = host.pending_images().len();
            host.set_status(if n == 1 {
                "Attached 1 clipboard image. Type a message and press Enter to send.".into()
            } else {
                format!("Attached {n} clipboard images. Press Enter to send them.")
            });
        }
        None => host.set_status(codetether_terminal_clipboard::clipboard_unavailable_message()),
    }
}
