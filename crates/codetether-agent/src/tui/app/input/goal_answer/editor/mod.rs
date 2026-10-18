//! Frontend type and user-journal selection for goal editing.
use codetether_goal_editor as draft;
pub(crate) use draft::Draft;
codetether_goal_editor::declare_frontend_goal_editor!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::models::ViewMode,
    codetether_editor::FileBuffer,
    codetether_goal_core::TaskState,
    codetether_goal_core::GoalEdit,
    crossterm::event::KeyEvent;
    crate::session::tasks::runtime::answer_review::read,
    crate::session::tasks::control::update_user,
    crate::tui::app::input::goal_answer::commands::restart,
);
#[cfg(test)]
pub(crate) use self::open::close;
pub(crate) use self::open::open;
pub(crate) use keys::handle;
pub(crate) use paste::paste;
#[cfg(test)]
mod tests;
