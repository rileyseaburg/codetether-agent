//! Native adapter for shared backspace routing.
use crate::tui::app::state::App;

pub async fn handle_backspace(app: &mut App) {
    codetether_tui_navigation::text::backspace(&mut super::text_host::Native(app));
}
