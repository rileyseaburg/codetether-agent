use crate::tui::app::state::App;
use crate::tui::ui::editor::EditorInput;

#[test]
fn approval_editor_blocks_file_switching() {
    let mut app = App::default();

    assert!(super::key::handle(&mut app, &EditorInput::OpenFinder));

    assert!(app.state.status.contains("File switching is disabled"));
}
