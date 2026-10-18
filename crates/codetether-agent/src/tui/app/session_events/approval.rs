//! Frontend callbacks for incoming approval presentation.
codetether_approval_view::declare_request_view!(
    crate::tui::app::state::App,
    codetether_approval_diagnostics::start,
    crate::tui::ui::trust_status::approval_guidance,
    crate::review::dispatch::start,
    crate::review::dispatch::Context,
);

#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
