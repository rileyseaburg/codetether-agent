//! Existing context selection regression.
use super::*;

#[test]
fn task_signature_picks_up_files_from_last_user_turn() {
    let msgs = vec![
        assistant("noise"),
        user("Please edit src/lib.rs and crates/foo/main.rs"),
        assistant("ok"),
    ];
    let task = task_signature(&msgs);
    assert!(task.files.iter().any(|f| f == "src/lib.rs"));
    assert!(task.files.iter().any(|f| f == "crates/foo/main.rs"));
}
