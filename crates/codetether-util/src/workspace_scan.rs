use std::path::Path;

pub fn is_pruned_workspace_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".hg"
            | ".svn"
            | ".codetether-agent"
            | ".codetether-worktrees"
            | "node_modules"
            | "target"
            | "dist"
            | "build"
            | ".next"
            | "vendor"
            | "__pycache__"
            | ".venv"
    )
}

pub fn path_has_pruned_component(path: &Path) -> bool {
    path.components()
        .filter_map(|c| c.as_os_str().to_str())
        .any(is_pruned_workspace_dir)
}

pub fn path_is_hidden(path: &Path) -> bool {
    path.components()
        .filter_map(|c| c.as_os_str().to_str())
        .any(|name| name.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use {
        super::{is_pruned_workspace_dir, path_has_pruned_component, path_is_hidden},
        std::path::Path,
    };

    #[test]
    fn prunes_codetether_worktrees() {
        assert!(is_pruned_workspace_dir(".codetether-worktrees"));
        assert!(path_has_pruned_component(Path::new(
            "tmp/.codetether-worktrees/session"
        )));
    }

    #[test]
    fn detects_hidden_components() {
        assert!(path_is_hidden(Path::new(".github/workflows/ci.yml")));
        assert!(!path_is_hidden(Path::new("src/main.rs")));
    }
}
