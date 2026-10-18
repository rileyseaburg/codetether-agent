//! Platform clipboard lookup for the shared input pipeline.

pub(super) fn local_content() -> Option<String> {
    codetether_terminal_clipboard::get_clipboard_text()
        .map(|text| text.replace("\r\n", "\n").replace('\r', "\n"))
        .or_else(|| {
            codetether_terminal_clipboard::get_clipboard_image().map(|image| image.data_url)
        })
}
