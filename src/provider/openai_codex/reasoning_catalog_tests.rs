//! Tests for authenticated Codex reasoning capability metadata.

use super::{is_gpt_56, supported_levels};

#[test]
fn catalog_distinguishes_ultra_and_max_models() {
    assert_eq!(supported_levels("gpt-6-astra"), ["low", "medium", "high"]);
    assert!(supported_levels("openai-codex/gpt-5.6-sol").contains(&"ultra"));
    assert_eq!(supported_levels("gpt-reserve").last(), Some(&"max"));
    assert_eq!(supported_levels("codex-auto-review").last(), Some(&"max"));
    assert!(!supported_levels("gpt-5.6-luna").contains(&"ultra"));
    assert_eq!(supported_levels("gpt-5.5").last(), Some(&"xhigh"));
}

#[test]
fn recognizes_codex_and_bedrock_gpt_56_names() {
    assert!(is_gpt_56("openai-codex/gpt-5.6-sol-fast:max"));
    assert!(is_gpt_56("bedrock/openai.gpt-5.6-terra"));
    assert!(!is_gpt_56("openai-codex/gpt-5.5"));
    assert!(!is_gpt_56("openai-codex/gpt-6-astra"));
}