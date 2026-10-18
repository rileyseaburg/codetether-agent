//! Native overlays operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::CloseSearch => {
            crate::tui::app::symbols::close_search(app);
        }
        Action::Status(value) => {
            app.state.status = value.clone();
        }
        Action::SetHelp(value) => {
            app.state.help.visible = *value;
        }
        Action::HelpReset => {
            app.state.help.scroll.offset = 0;
        }
        Action::CloseModelPicker => {
            crate::tui::app::model_picker::close_model_picker(app);
        }
        Action::ClearSessionFilter => {
            app.state.clear_session_filter();
        }
        Action::ReturnToChat => {
            crate::tui::app::session_sync::return_to_chat(app);
        }
        Action::DisableFilePicker => {
            app.state.file_picker.active = false;
        }
        Action::SwarmExitDetail => {
            app.state.swarm.exit_detail();
        }
        Action::RalphExitDetail => {
            app.state.ralph.exit_detail();
        }
        Action::BusExitFilter => {
            app.state.bus_log.exit_filter_mode();
        }
        Action::BusExitDetail => {
            app.state.bus_log.exit_detail();
        }
        Action::NormalInput => {
            app.state.composer.mode = crate::tui::models::InputMode::Normal;
        }
        Action::SubagentDetail(value) => {
            app.state.agent_tree.detail_mode = *value;
        }
        _ => return false,
    }
    true
}
