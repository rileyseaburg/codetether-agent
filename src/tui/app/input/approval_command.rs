use crate::tui::app::state::App;

pub(crate) fn run(app: &mut App, prompt: &str) -> bool {
    dispatch::run(app, prompt)
}

#[path = "approval_command_dispatch.rs"]
mod dispatch;
#[path = "approval_command_intent.rs"]
mod intent;
#[path = "approval_command_parse.rs"]
mod parse;
#[cfg(test)]
#[path = "approval_command_queue_tests.rs"]
mod queue_tests;
#[path = "approval_command_result.rs"]
mod result;
#[path = "approval_command_store.rs"]
mod store;
#[cfg(test)]
#[path = "approval_command_test_support.rs"]
mod test_support;
