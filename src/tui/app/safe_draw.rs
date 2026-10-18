//! Frontend resource and callback selection for guarded terminal drawing.
codetether_terminal_runtime::declare_frontend_safe_draw!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionView,
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;
    crate::telemetry::crash_context::record_tui,
    crate::tui::ui::main::ui,
);
