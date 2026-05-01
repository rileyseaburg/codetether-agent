//! Windows-specific computer use implementations.

mod apps;
mod input;
mod request;
mod snapshot;
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
        ComputerUseAction::RightClick => input::handle_right_click(input).await,
        ComputerUseAction::DoubleClick => input::handle_double_click(input).await,
        ComputerUseAction::Drag => input::handle_drag(input).await,
        ComputerUseAction::TypeText => input::handle_type_text(input).await,
        ComputerUseAction::PressKey => input::handle_press_key(input).await,
        ComputerUseAction::Scroll => input::handle_scroll(input).await,
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
