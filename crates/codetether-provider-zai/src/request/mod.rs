//! Buffered completion payload assembly with explicit field conventions.
//!
//! [encode_buffered] accepts already-converted history and thinking settings.
//! [BufferedFormat] keeps application and injected-client field behavior distinct.
use crate::CompletionRequest;
use serde_json::Value;
mod controls;
mod encode;
mod format;
mod tools;
pub use encode::encode_buffered;
pub use format::BufferedFormat;
pub(crate) use tools::tools;
#[cfg(test)]
mod tests;

pub(crate) fn body(request: &CompletionRequest) -> Value {
    encode_buffered(
        request,
        crate::messages::convert(&request.messages, false),
        tools(request),
        crate::thinking::thinking_config_for_request(request),
        BufferedFormat::ExplicitControls,
    )
}
