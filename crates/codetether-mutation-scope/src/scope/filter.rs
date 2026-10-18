//! Retain only bounded claims without denying unrelated command execution.

use super::MutationScope;

/// Remove whole-workspace targets while retaining bounded file/subtree claims.
/// # Arguments
/// `scope` supplies the root and requested paths.
/// # Returns
/// A filtered scope, or None when no bounded target remains.
/// # Examples
/// ```
/// let scope = codetether_mutation_scope::scope::MutationScope {
///     workspace: "/repo".into(), paths: vec![std::path::PathBuf::new()],
/// };
/// assert!(codetether_mutation_scope::scope::scoped_only(scope).is_none());
/// ```
pub fn scoped_only(mut scope: MutationScope) -> Option<MutationScope> {
    scope
        .paths
        .retain(|path| !crate::workspace_claim(&scope.workspace, path));
    (!scope.paths.is_empty()).then_some(scope)
}
