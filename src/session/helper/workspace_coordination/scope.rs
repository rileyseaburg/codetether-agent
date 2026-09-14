//! Repository-root normalization for mutation lease requests.

use std::path::{Path, PathBuf};

#[path = "scope/filter.rs"]
mod filter;
#[path = "scope/root.rs"]
mod root;

pub(super) use filter::scoped_only;

#[cfg(test)]
#[path = "scope/checkout_tests.rs"]
mod checkout_tests;
#[cfg(test)]
#[path = "scope/filter_tests.rs"]
mod filter_tests;

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
