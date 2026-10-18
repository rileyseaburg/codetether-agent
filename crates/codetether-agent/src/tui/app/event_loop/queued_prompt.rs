//! Frontend selection for deferred chat submission.
codetether_chat_submit::declare_frontend_queued_prompt!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::tui::app::input::goal_answer::commands::drain,
    crate::tui::app::input::goal_answer::deliver,
    crate::tui::app::input::handle_enter;
    crate::tui::app::state::prompt_queue::has_pending,
    crate::session::tasks::runtime::answer_review::held,
    crate::tui::app::state::prompt_queue::take,
);
