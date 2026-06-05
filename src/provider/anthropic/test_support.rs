//! Test fixtures for Anthropic provider helper tests.
//!
//! This module contains small constructors and JSON inspection helpers shared by
//! Anthropic provider unit tests. The items are compiled only for tests and
//! provide focused fixtures for message conversion, tool conversion, and prompt
//! cache metadata assertions.

#[cfg(test)]
use serde_json::{Value, json};

#[cfg(test)]
use crate::provider::{ContentPart, Message, Role, ToolDefinition};

/// Build a provider-neutral system message for Anthropic conversion tests.
///
/// # Parameters
///
/// * `text` - System instruction text to place in a single text content block.
///
/// # Returns
///
/// A [`Message`] with [`Role::System`] and one [`ContentPart::Text`] containing
/// the supplied text.
#[cfg(test)]
pub(crate) fn system_message(text: &str) -> Message {
    Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

/// Build a provider-neutral user message for Anthropic conversion tests.
///
/// # Parameters
///
/// * `text` - User message text to place in a single text content block.
///
/// # Returns
///
/// A [`Message`] with [`Role::User`] and one [`ContentPart::Text`] containing
/// the supplied text.
#[cfg(test)]
pub(crate) fn user_message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

/// Build a minimal weather tool definition used by conversion tests.
///
/// The fixture uses a stable tool name and a minimal object schema so tests can
/// focus on how tool definitions are translated rather than on schema content.
///
/// # Returns
///
/// A [`ToolDefinition`] named `get_weather` with an object JSON schema.
#[cfg(test)]
pub(crate) fn weather_tool() -> ToolDefinition {
    ToolDefinition {
        name: "get_weather".to_string(),
        description: "Get weather".to_string(),
        parameters: json!({"type": "object"}),
    }
}

/// Read the prompt-cache control type from a serialized content block.
///
/// # Parameters
///
/// * `value` - Optional JSON value expected to contain a `cache_control` object.
///
/// # Returns
///
/// The nested `cache_control.type` string when present and shaped as expected;
/// otherwise `None`.
#[cfg(test)]
pub(crate) fn cache_type(value: Option<&Value>) -> Option<&str> {
    value?.get("cache_control")?.get("type")?.as_str()
}

/// Read the prompt-cache control type from the last serialized message content block.
///
/// This helper is used by tests that need to verify which converted message
/// block receives Anthropic prompt-cache metadata.
///
/// # Parameters
///
/// * `value` - Optional serialized message JSON expected to contain a `content`
/// array.
///
/// # Returns
///
/// The `cache_control.type` string from the final content block when present;
/// otherwise `None`.
#[cfg(test)]
pub(crate) fn message_cache_type(value: Option<&Value>) -> Option<&str> {
    cache_type(value?.get("content")?.as_array()?.last())
}
