//! Windows-specific computer use implementations.

mod apps;
mod input;
mod ps;
mod request;
mod snapshot;
mod status;

use crate::tool::computer_use::{input::ComputerUseAction, input::ComputerUseInput};

pub async fn dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    match input.action {
        ComputerUseAction::Status => status::handle_status(),
        ComputerUseAction::ListApps => apps::handle_list_apps(),
        ComputerUseAction::RequestApp => request::handle_request_app(input),
        ComputerUseAction::Snapshot => snapshot::handle_snapshot(input),
        ComputerUseAction::Click => input::handle_click(input),
        ComputerUseAction::TypeText => input::handle_type_text(input),
        ComputerUseAction::PressKey => input::handle_press_key(input),
        ComputerUseAction::Scroll => input::handle_scroll(input),
        ComputerUseAction::Stop => input::handle_stop(),
    }
}
