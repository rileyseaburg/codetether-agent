//! Frontend session directory binding for isolated prompt checkouts.
/// Declare worktree creation using a concrete session and branch lookup.
/// # Examples
/// See the frontend input worktree module for its selected types and callback.
#[macro_export]
macro_rules! declare_frontend_prompt_worktree {
    ($session:path, $branch:path $(,)?) => {
        use ::std::path::Path;
        use $branch as current_branch;
        use $session as Session;
        pub(crate) async fn create_worktree(
            cwd: &Path,
            session: &mut Session,
        ) -> Option<WorktreeState> {
            $crate::prompt::create_worktree(cwd, current_branch, |path| {
                session.metadata.directory = Some(path);
            })
            .await
        }
    };
}
