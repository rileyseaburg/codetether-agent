//! Git/FS helpers backing [`super::workspace::WorkspaceSnapshot`].


pub fn should_skip_entry(name: &str) -> bool {
    crate::workspace_scan::is_pruned_workspace_dir(name)
}
pub use codetether_git_view::detect_git_status;
