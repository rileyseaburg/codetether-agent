//! Frontend prompt execution type and scope selection.
pub(crate) use codetether_github_pr::worktree::handle_worktree_result;
codetether_session_runtime::declare_frontend_prompt_run!(
    crate::session::Session,
    crate::session::ImageAttachment,
    crate::session::SessionEvent,
    crate::provider::ProviderRegistry;
    codetether_question::runtime::scope,
);
