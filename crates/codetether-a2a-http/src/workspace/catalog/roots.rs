//! Workspace catalog roots handling.

pub(super) fn is_under_workspace_root(path: &str, roots: &[String]) -> bool {
    let path = std::path::Path::new(path);
    roots
        .iter()
        .map(std::path::Path::new)
        .any(|root| path == root || path.starts_with(root))
}
