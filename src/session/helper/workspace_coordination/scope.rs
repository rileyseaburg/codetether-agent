//! Repository-root normalization for mutation lease requests.

use std::path::{Path, PathBuf};

#[path = "scope/root.rs"]
mod root;

pub(super) struct MutationScope {
    pub workspace: PathBuf,
    pub paths: Vec<PathBuf>,
}

pub(super) fn resolve(parent: &Path, paths: Vec<PathBuf>) -> MutationScope {
    let absolute = paths
        .into_iter()
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                parent.join(path)
            }
        })
        .collect::<Vec<_>>();
    let Some(root) = root::shared(&absolute).or_else(|| root::single_directory(&absolute)) else {
        return MutationScope {
            workspace: parent.into(),
            paths: absolute,
        };
    };
    let paths = absolute
        .into_iter()
        .map(|path| path.strip_prefix(&root).unwrap_or(&path).into())
        .collect();
    MutationScope {
        workspace: root,
        paths,
    }
}

pub(super) fn too_broad(scope: &MutationScope) -> bool {
    let root_claim = scope.paths.iter().any(|path| path.as_os_str().is_empty());
    root_claim && super::scope_error::unsafe_workspace(&scope.workspace)
}
