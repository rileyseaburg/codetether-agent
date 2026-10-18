//! Frontend type and callback selection for active-turn steering.
codetether_chat_submit::declare_frontend_steering!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::ImageAttachment,
    crate::session::helper::steering::SteeringInput,
    codetether_chat_model::ChatMessage;
    crate::tui::app::input::pasted_text::expand_paste_placeholders,
    crate::tui::app::state::prompt_queue::store,
);
