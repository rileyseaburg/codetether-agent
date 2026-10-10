use super::super::{WorktreeCleanupState, parse};

#[test]
fn parses_nul_delimited_porcelain_records() {
    let raw = b"worktree /repo\0HEAD aaa\0branch refs/heads/main\0\0\
worktree /trees/task\0HEAD bbb\0detached\0locked reason\0\0";

    let records = parse::records(raw);

    assert_eq!(records.len(), 2);
    assert!(records[0].primary);
    assert_eq!(records[0].branch.as_deref(), Some("main"));
    assert!(records[1].locked);
    assert_eq!(WorktreeCleanupState::Locked.as_str(), "locked");
}
