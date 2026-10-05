use super::{parse_fast_alias, supports_fast};

#[test]
fn recognizes_new_codex_fast_models() {
    assert_eq!(parse_fast_alias("gpt-5.6-sol-fast"), Some("gpt-5.6-sol"));
    assert_eq!(parse_fast_alias("gpt-6-astra-fast"), Some("gpt-6-astra"));
    assert_eq!(parse_fast_alias("gpt-reserve-fast"), Some("gpt-reserve"));
    assert!(supports_fast("openai-codex/gpt-5.6-terra"));
    assert!(supports_fast("openai-codex/codex-auto-review"));
    assert!(!supports_fast("openai-codex/gpt-5.4-mini"));
    assert!(!supports_fast("openai-codex/gpt-5.3-codex"));
}

#[test]
fn ultrafast_is_astra_only_and_sol61_has_fast() {
    assert_eq!(super::suffixes("gpt-6-astra"), vec!["-fast", "-ultrafast"]);
    assert_eq!(super::suffixes("gpt-6.1-sol"), vec!["-fast"]);
}
