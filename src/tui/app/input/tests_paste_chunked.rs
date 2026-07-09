//! Emulate tmux splitting a large image data-URL paste into chunks.

#[cfg(test)]
mod tests {
    use base64::Engine;

    use crate::tui::app::input::handle_paste;
    use crate::tui::app::input::image_sidecar_recover::recover_pasted_images;
    use crate::tui::app::state::App;
    use crate::tui::models::ViewMode;

    /// Build a screenshot-sized data URL (~120 KiB of base64).
    fn big_data_url() -> String {
        let bytes = vec![0u8; 90 * 1024];
        let payload = base64::engine::general_purpose::STANDARD.encode(bytes);
        format!("data:image/png;base64,{payload}")
    }

    /// Feed `text` to the paste handler in `chunk`-byte pieces, the way
    /// tmux delivers one oversized paste as several paste events.
    async fn paste_in_chunks(app: &mut App, text: &str, chunk: usize) {
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let end = (i + chunk).min(bytes.len());
            handle_paste(app, std::str::from_utf8(&bytes[i..end]).unwrap()).await;
            i = end;
        }
    }

    #[tokio::test]
    async fn chunked_data_url_paste_still_attaches_image() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        let url = big_data_url();

        paste_in_chunks(&mut app, &url, 4096).await;
        // Same recovery call that handle_enter_chat runs.
        recover_pasted_images(&mut app);

        assert_eq!(
            app.state.pending_images.len(),
            1,
            "image lost: input_len={} sidecar_pastes={} status={:?}",
            app.state.input.len(),
            app.state.pending_text_pastes.len(),
            app.state.status,
        );
    }
}
