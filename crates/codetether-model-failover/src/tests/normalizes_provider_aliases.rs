//! Provider-error regression retained from the native smart switch module.
use super::super::*;

#[test]
fn normalizes_provider_aliases() {
    assert_eq!(normalize_provider_alias("z-ai"), "zai");
    assert_eq!(normalize_provider_alias("ZhipuAI"), "zai");
    assert_eq!(normalize_provider_alias("openai"), "openai");
}
