//! Copilot chat-completion response decoding.

#[path = "copilot_response/convert.rs"]
mod convert;
#[path = "copilot_response/finish.rs"]
mod finish;
#[cfg(test)]
#[path = "copilot_response/tests.rs"]
mod tests;
#[path = "copilot_response/types.rs"]
mod types;
#[path = "copilot_response/usage.rs"]
mod usage;
pub(super) use convert::to_completion_response;
pub(super) use types::CopilotResponse;
