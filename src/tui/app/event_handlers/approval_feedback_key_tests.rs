use crate::approval::{LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

struct QueueGuard;
impl Drop for QueueGuard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

#[test]
fn edit_opens_proposed_source_without_writing_it() {
    let _lock = lock_env();
    approval_queue::reset();
    let _guard = QueueGuard;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("src/lib.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "fn old() {}\n").unwrap();
    let request = LiveApprovalRequest::new(
        "approval-1".into(),
        "call-1".into(),
        "apply_patch".into(),
        "write".into(),
        "src/lib.rs".into(),
        "review".into(),
    )
    .with_preview(
        "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1,1 +1,1 @@\n-fn old() {}\n+fn new() {}\n".into(),
    );
    approval_queue::push(request);
    let mut app = App::default();

    assert!(super::edit(&mut app, dir.path()));

    assert_eq!(app.state.editor.as_ref().unwrap().text(), "fn new() {}");
    assert!(app.state.approval_edit.is_some());
    assert_eq!(std::fs::read_to_string(path).unwrap(), "fn old() {}\n");
}
