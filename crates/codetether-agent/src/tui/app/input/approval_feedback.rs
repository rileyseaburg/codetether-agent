//! Frontend callback selection for pending approval text feedback.
codetether_approval_view::declare_frontend_approval_feedback!(
    crate::tui::app::state::App;
    crate::tui::app::input::image_sidecar_recover::recover_pasted_images,
    crate::tui::app::state::approval_queue::active_id,
    crate::tui::app::input::approval_command::run,
);
#[cfg(test)]
#[path = "approval_feedback_tests.rs"]
mod tests;
