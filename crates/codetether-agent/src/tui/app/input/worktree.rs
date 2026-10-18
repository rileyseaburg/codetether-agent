//! Frontend type and branch selection for prompt worktrees.
pub(crate) use codetether_worktree::prompt::WorktreeState;
codetether_worktree::declare_frontend_prompt_worktree!(
    crate::session::Session,
    codetether_git_view::current_branch,
);
