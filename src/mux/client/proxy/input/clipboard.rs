//! Platform clipboard lookup for the shared input pipeline.

pub(super) fn local_content() -> Option<String> {
    crate::tui::clipboard::get_clipboard_text()
        .map(|text| text.replace("\r\n", "\n").replace('\r', "\n"))
        .or_else(|| crate::tui::clipboard::get_clipboard_image().map(|image| image.data_url))
}
