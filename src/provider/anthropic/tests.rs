//! Unit tests for Anthropic provider helpers.

use crate::provider::Provider;

#[test]
fn adds_cache_control_to_last_tool_system_and_message_block() {
    let messages = vec![
        super::test_support::system_message("static instruction"),
        super::test_support::user_message("dynamic input"),
    ];
    let (system, converted_messages) = super::convert::messages(&messages, true);
    let mut converted_tools =
        super::convert_tools::tools(&[super::test_support::weather_tool()], true);
    assert_eq!(
        super::test_support::cache_type(system.as_ref().unwrap().last()),
        Some("ephemeral")
    );
    assert_eq!(
        super::test_support::message_cache_type(converted_messages.last()),
        Some("ephemeral")
    );
    assert_eq!(
        super::test_support::cache_type(converted_tools.pop().as_ref()),
        Some("ephemeral")
    );
}

#[test]
fn minimax_provider_name_enables_prompt_caching_by_default() {
    let provider = super::AnthropicProvider::with_base_url(
        "test-key".to_string(),
        "https://api.minimax.io/anthropic".to_string(),
        "minimax",
    )
    .expect("provider should initialize");
    assert_eq!(provider.name(), "minimax");
    assert!(provider.enable_prompt_caching);
}

#[test]
fn safe_char_prefix_handles_multibyte_characters() {
    let s = "abc✓def";
    assert_eq!(super::complete::safe_char_prefix(s, 4), "abc✓");
    assert_eq!(super::complete::safe_char_prefix(s, 7), s);
}
