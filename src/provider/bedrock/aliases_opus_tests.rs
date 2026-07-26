//! Tests for [`crate::provider::bedrock::aliases_opus::resolve_opus_alias`].

use super::resolve_opus_alias;
use crate::provider::bedrock::resolve_model_id;

#[test]
fn opus_5_resolves_to_global_profile() {
    assert_eq!(
        resolve_opus_alias("claude-opus-5"),
        Some("global.anthropic.claude-opus-5")
    );
    assert_eq!(
        resolve_opus_alias("opus-5"),
        Some("global.anthropic.claude-opus-5")
    );
}

#[test]
fn opus_5_resolves_through_public_entry_point() {
    assert_eq!(
        resolve_model_id("claude-opus-5"),
        "global.anthropic.claude-opus-5"
    );
    // Already-canonical IDs pass through unchanged.
    assert_eq!(
        resolve_model_id("global.anthropic.claude-opus-5"),
        "global.anthropic.claude-opus-5"
    );
}

#[test]
fn older_opus_aliases_are_preserved() {
    assert_eq!(
        resolve_model_id("claude-opus-4-7"),
        "us.anthropic.claude-opus-4-7"
    );
    assert_eq!(
        resolve_model_id("claude-opus-4-6"),
        "us.anthropic.claude-opus-4-6-v1"
    );
    assert_eq!(
        resolve_model_id("claude-opus-4"),
        "us.anthropic.claude-opus-4-20250514-v1:0"
    );
}

#[test]
fn non_opus_models_return_none() {
    assert_eq!(resolve_opus_alias("claude-sonnet-4-6"), None);
    assert_eq!(resolve_opus_alias("nova-lite"), None);
}
