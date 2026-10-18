//! Repository-root normalization for mutation lease requests.

use std::path::{Path, PathBuf};

#[path = "scope/filter.rs"]
mod filter;
#[path = "scope/root.rs"]
mod root;

pub use filter::scoped_only;

#[cfg(test)]
#[path = "scope/checkout_tests.rs"]
mod checkout_tests;
#[cfg(test)]
#[path = "scope/filter_tests.rs"]
mod filter_tests;

/// Checkout root and rebased paths covered by a mutation request.
/// # Examples
/// ```
/// let scope = codetether_mutation_scope::scope::MutationScope {
///     workspace: "/repo".into(), paths: vec!["a.rs".into()],
/// };
/// assert_eq!(scope.paths.len(), 1);
/// ```
pub struct MutationScope {
    /// Repository root or the original workspace when no root is found.
    pub workspace: PathBuf,
    /// Relative paths when rebased, otherwise absolute targets.
    pub paths: Vec<PathBuf>,
}

/// Resolve target paths to their shared repository or single directory root.
/// # Arguments
/// `parent` anchors relative targets; `paths` contains requested mutation paths.
/// # Returns
/// The inferred workspace root and rebased target paths.
/// # Examples
/// ```no_run
/// let scope = codetether_mutation_scope::scope::resolve(std::path::Path::new("/repo"), vec!["a.rs".into()]);
/// assert_eq!(scope.paths.len(), 1);
/// ```
pub fn resolve(parent: &Path, paths: Vec<PathBuf>) -> MutationScope {
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
