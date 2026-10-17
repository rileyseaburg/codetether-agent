//! config for Z.AI thinking requests.
use super::should_disable_thinking_for_request;
use crate::CompletionRequest;
use serde_json::{Value, json};
/// Build the thinking configuration from the shared request policy.
///
/// # Arguments
/// * request - Request whose messages and token limit determine the policy.
///
/// # Returns
/// Disabled thinking, or enabled thinking with cleared historical reasoning.
///
/// # Examples
///
///     use codetether_provider_zai::{CompletionRequest, thinking};
///     fn inspect(request: &CompletionRequest) {
///         assert!(thinking::thinking_config_for_request(request).get("type").is_some());
///     }
pub fn thinking_config_for_request(request: &CompletionRequest) -> Value {
    if should_disable_thinking_for_request(request) {
        json!({"type": "disabled"})
    } else {
        json!({
            "type": "enabled",
            "clear_thinking": true
        })
    }
}
