use crate::tui::app::state::App;

pub(crate) fn run(app: &mut App, prompt: &str) -> bool {
    dispatch::run(app, prompt)
}

pub(crate) use release::all as release_all;

#[path = "approval_command_dispatch.rs"]
mod dispatch;
#[path = "approval_command_intent.rs"]
mod intent;
#[path = "approval_command_message.rs"]
mod message;
#[cfg(test)]
#[path = "approval_command_order_support.rs"]
mod order_support;
#[cfg(test)]
#[path = "approval_command_order_tests.rs"]
mod order_tests;
#[path = "approval_command_parse.rs"]
mod parse;
#[cfg(test)]
#[path = "approval_command_queue_tests.rs"]
mod queue_tests;
#[path = "approval_command_release.rs"]
mod release;
#[path = "approval_command_result.rs"]
mod result;
#[path = "approval_command_store.rs"]
mod store;
#[cfg(test)]
#[path = "approval_command_test_support.rs"]
mod test_support;
