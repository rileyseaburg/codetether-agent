//! Frontend attachment and publication selection for prompt finalization.
#[path = "chat_submit_goal.rs"]
mod goal;
codetether_chat_submit::declare_frontend_prompt_finish!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::ImageAttachment,
    crate::tui::app::input::pasted_text::PendingTextPaste;
    crate::tui::app::input::chat_submit_finish::goal::adopt,
    crate::tui::app::input::chat_helpers::push_user_messages,
    crate::tui::app::input::chat_submit_dispatch::dispatch_prompt,
    crate::tui::app::input::pasted_text::expand_paste_placeholders,
);
