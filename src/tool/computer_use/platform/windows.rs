//! Windows-specific computer use implementations.

mod apps;
mod input;
mod request;
mod snapshot;
mod snapshot_output;
mod status;

use crate::tool::computer_use::{input::ComputerUseAction, input::ComputerUseInput, response};

pub async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    if gated_action(input) {
        return Ok(response::error_result(
            "request_app is advisory only; app-targeted actions are not enforced yet",
        ));
    }
    match input.action {
        ComputerUseAction::Status => status::handle_status(),
        ComputerUseAction::ListApps => apps::handle_list_apps().await,
        ComputerUseAction::RequestApp => request::handle_request_app(input),
        ComputerUseAction::Snapshot => snapshot::handle_snapshot(input).await,
        ComputerUseAction::WindowSnapshot => snapshot::handle_window_snapshot(input).await,
        ComputerUseAction::Click => input::handle_click(input).await,
        ComputerUseAction::ClickClient => input::handle_click_client(input).await,
        ComputerUseAction::RightClick => input::handle_right_click(input).await,
        ComputerUseAction::DoubleClick => input::handle_double_click(input).await,
        ComputerUseAction::Drag => input::handle_drag(input).await,
        ComputerUseAction::MouseDown => input::handle_mouse_down(input).await,
        ComputerUseAction::MouseMove => input::handle_mouse_move(input).await,
        ComputerUseAction::MouseUp => input::handle_mouse_up(input).await,
        ComputerUseAction::TypeText => input::handle_type_text(input).await,
        ComputerUseAction::SetText => input::handle_set_text(input).await,
        ComputerUseAction::PressKey => input::handle_press_key(input).await,
        ComputerUseAction::Scroll => input::handle_scroll(input).await,
        ComputerUseAction::FocusViewport => input::handle_focus_viewport(input).await,
        ComputerUseAction::BlenderSelectFrame => input::handle_blender_select_frame(input).await,
        ComputerUseAction::BringToFront => input::handle_bring_to_front(input).await,
        ComputerUseAction::WaitMs => input::handle_wait_ms(input).await,
        ComputerUseAction::Stop => input::handle_stop(),
    }
}

fn gated_action(input: &ComputerUseInput) -> bool {
    !matches!(
        input.action,
        ComputerUseAction::Status | ComputerUseAction::ListApps | ComputerUseAction::RequestApp
    ) && (input.app.is_some() || input.window_title_contains.is_some())
}
