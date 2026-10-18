//! Native composer binding for shared image-sidecar recovery.
use crate::tui::app::state::App;
pub(crate) fn recover_pasted_images(app: &mut App) -> usize {
    let mut state = codetether_paste_buffer::image_recovery::State {
        composer: codetether_image::paste::State {
            images: &mut app.state.pending_images,
            input: &mut app.state.input,
            cursor: &mut app.state.input_cursor,
            status: &mut app.state.status,
        },
        pastes: &mut app.state.pending_text_pastes,
    };
    codetether_paste_buffer::image_recovery::recover_pasted_images(&mut state)
}
