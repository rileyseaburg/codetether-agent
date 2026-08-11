use super::loading;
use crate::tui::app::state::App;

#[test]
fn loading_state_is_ready_for_the_first_frame() {
    let mut app = App::default();

    loading(&mut app, std::path::Path::new("/workspace"), true);

    assert_eq!(app.state.cwd_display, "/workspace");
    assert!(app.state.allow_network);
    assert_eq!(app.state.status, "Loading providers and workspace...");
}
