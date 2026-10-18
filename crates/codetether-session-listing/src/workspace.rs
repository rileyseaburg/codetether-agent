use std::path::{Path, PathBuf};

pub(super) fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub(super) fn matches_workspace(session_dir: Option<&Path>, workspace: Option<&Path>) -> bool {
    let Some(workspace) = workspace else {
        return true;
    };
    let Some(session_dir) = session_dir else {
        return false;
    };
    if session_dir == workspace {
        return true;
    }
    match session_dir.canonicalize() {
        Ok(dir) => dir == workspace,
        Err(_) => {
            session_dir == workspace
                || workspace.starts_with(session_dir)
                || session_dir.starts_with(workspace)
        }
    }
}
