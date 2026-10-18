//! Frontend runtime selection for automatic prompt-to-goal adoption.
codetether_chat_submit::declare_frontend_goal_adoption!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::session::tasks::runtime::adopt_prompt,
);
