//! Shared AppState initialization with the existing component constructors.
codetether_tui_models::impl_app_state_default!(
    super::AppState;
    bus = crate::tui::bus_log::BusLogState::new,
    swarm = crate::tui::swarm_view::SwarmViewState::new,
    audit = crate::tui::audit_view::AuditViewState::default,
    git = super::git_state::GitViewState::default,
    ralph = crate::tui::ralph_view::RalphViewState::new,
    audit_loop = crate::tui::audit_loop_view::audit_loop_state::AuditLoopState::new,
    symbols = crate::tui::symbol_search::SymbolSearchState::new,
    help = codetether_help_view::HelpState::default,
    models = codetether_model_picker::ModelPickerState::default,
    mcp = crate::tui::app::mcp::TuiMcpRegistry::new,
    autochat = super::super::autochat::state::AutochatState::default,
    workspace = crate::tui::models::WorkspaceSnapshot::default,
    layout = crate::tui::ui::webview::layout_mode::ChatLayoutMode::default,
    forage = crate::tui::forage_run::ForageState::new
);
