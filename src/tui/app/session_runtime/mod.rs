//! Move-based TUI session runtime.

use codetether_session_control as active_cancel;
#[cfg(test)]
mod active_turn;
mod command;
mod dispatch_host;
mod handle;
mod loop_submit;
mod notice;
mod prompt_result;
mod runtime;
mod slot;
mod view;

#[cfg(test)]
#[path = "runtime_completion_tests.rs"]
mod completion_tests;
#[cfg(test)]
mod tests;

pub(crate) use command::{PromptRequest, SessionCommand};
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
