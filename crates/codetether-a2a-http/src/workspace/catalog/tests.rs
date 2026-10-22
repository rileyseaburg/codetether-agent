use super::*;
#[test]
fn path_under_root_matches() {
    let roots = vec!["/workspace".to_string()];
    assert!(is_under_workspace_root("/workspace", &roots));
    assert!(is_under_workspace_root("/workspace/spotlessbinco", &roots));
    assert!(!is_under_workspace_root("/other/path", &roots));
}
#[test]
fn empty_roots_match_nothing() {
    assert!(!is_under_workspace_root("/workspace/foo", &[]));
}
