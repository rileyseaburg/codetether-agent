//! Retain only bounded claims without denying unrelated command execution.

use super::MutationScope;

pub(in crate::session::helper::workspace_coordination) fn scoped_only(
    mut scope: MutationScope,
) -> Option<MutationScope> {
    scope
        .paths
        .retain(|path| !crate::mux::lease::workspace_claim(&scope.workspace, path));
    (!scope.paths.is_empty()).then_some(scope)
}
