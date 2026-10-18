//! Move-based TUI session runtime.

use codetether_session_control as active_cancel;
#[cfg(test)]
mod active_turn;
mod command;
mod handle;
mod loop_submit;
mod notice;
mod prompt_result;
mod slot;
mod view;

#[cfg(test)]
#[path = "runtime_completion_tests.rs"]
mod completion_tests;
#[cfg(test)]
mod tests;

#[cfg(not(test))]
pub(crate) use command::PromptRequest;
pub(crate) use handle::TuiSessionHandle;
pub(crate) use notice::SessionNotice;
pub(crate) use runtime::spawn;
pub(crate) use slot::SessionSlot;
pub use view::SessionView;

codetether_session_runtime::declare_frontend_execution!(
    crate::session::Session,
    crate::provider::ProviderRegistry,
    crate::session::ImageAttachment,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::SessionNotice,
    crate::tui::app::input::worktree::WorktreeState,
    crate::tui::app::session_runtime::prompt_result::PromptRunResult,
    crate::tui::app::session_runtime::PromptRequest,
    codetether_session_control::ActiveCancel;
    crate::tui::app::input::worktree::create_worktree,
    crate::tui::app::input::worktree_result::run_prompt,
    crate::tui::app::session_runtime::prompt_result::notice,
    crate::tui::app::input::worktree_result::handle_worktree_result,
);

codetether_session_runtime::declare_frontend_runtime_dispatch!(
    crate::tui::app::session_runtime::PromptRequest,
    crate::tui::app::session_runtime::SessionNotice,
    crate::session::SessionEvent,
    codetether_session_control::ActiveCancel,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::session::tasks::runtime::answer_review::held,
    crate::tui::app::session_runtime::execute::run,
);

#[cfg(test)]
include!("mod_test_imports.rs");
