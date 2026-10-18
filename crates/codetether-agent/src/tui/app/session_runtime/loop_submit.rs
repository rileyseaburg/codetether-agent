//! Frontend binding for shared ordered prompt submission.
codetether_session_runtime::declare_frontend_loop_submit!(
    crate::tui::app::session_runtime::PromptRequest,
    crate::tui::app::session_runtime::SessionNotice,
    crate::session::SessionEvent,
    codetether_session_control::ActiveCancel,
    crate::tui::app::session_runtime::dispatch_host::Native,
);
#[cfg(test)]
#[path = "answer_review_submit_tests.rs"]
mod tests;
