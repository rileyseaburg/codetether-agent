//! Pause goal work, answer user questions without tools, then await acceptance.

mod answer;
pub(crate) mod commands;
#[cfg(test)]
mod continuation;
pub(crate) mod editor;
mod submit;

pub(crate) use answer::deliver;
pub(super) use submit::intercept;

codetether_goal_answer::declare_frontend_answer_hosts!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::tasks::TaskState,
    crate::session::tasks::AnswerReviewAction;
    crate::session::tasks::runtime::answer_review::read,
    crate::session::tasks::runtime::answer_review::held,
    crate::session::tasks::runtime::answer_review::begin,
    crate::session::tasks::runtime::answer_review::record;
    crate::tui::app::input::goal_answer::commands::restart,
    crate::tui::app::ask::run_ask,
);
