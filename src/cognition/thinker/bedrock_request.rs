//! Compatibility exports for shared Bedrock request construction.
#[cfg(test)]
use super::ThinkerConfig;
pub(super) use codetether_cognition_core::thinker::protocol::bedrock_request::{body, url};
#[cfg(test)]
#[path = "bedrock_request_tests.rs"]
mod tests;
