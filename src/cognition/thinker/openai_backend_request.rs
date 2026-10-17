//! Compatibility export for shared OpenAI-compatible request construction.
#[cfg(test)]
use super::ThinkerConfig;
pub(super) use codetether_cognition_core::thinker::protocol::openai_request::body;
#[cfg(test)]
#[path = "openai_backend_request_tests.rs"]
mod tests;
