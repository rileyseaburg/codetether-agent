//! Existing model selection regression coverage.
use super::*;

#[test]
fn provider_alias_normalizes_zhipu_and_z_ai() {
    assert_eq!(normalize_provider_name("zhipuai"), "zai");
    assert_eq!(normalize_provider_name("z-ai"), "zai");
    assert_eq!(normalize_provider_name("openrouter"), "openrouter");
}
