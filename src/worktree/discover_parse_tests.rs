use super::discover_parse::parse_worktree_list;
use std::path::Path;

#[test]
fn parses_codetether_worktree_from_porcelain() {
    let output = "\
worktree /repo/.codetether-worktrees/task_1
HEAD abc
branch refs/heads/codetether/task_1
";
    let infos = parse_worktree_list(output, Path::new("/repo/.codetether-worktrees"));
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].name, "task_1");
    assert_eq!(infos[0].branch, "codetether/task_1");
}

#[test]
fn ignores_unrelated_worktree() {
    let output = "\
worktree /repo/other
HEAD abc
branch refs/heads/main
";
    let infos = parse_worktree_list(output, Path::new("/repo/.codetether-worktrees"));
    assert!(infos.is_empty());
}
