//! Copilot chat-completion response decoding.

#[path = "convert.rs"]
mod convert;
#[path = "finish.rs"]
mod finish;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
mod types;
#[path = "usage.rs"]
mod usage;
pub(crate) use convert::to_completion_response;
pub(crate) use types::CopilotResponse;
