//! Alias decoding does not depend on the process's model discovery snapshot.
use super::*;

#[test]
fn ultrafast_alias_decodes_without_catalog_entry() {
    let base = "gpt-future-ultrafast-probe";
    assert!(!service_tier_catalog::supports_ultrafast(base));
    assert_eq!(
        parse_service_tier_model_alias(&format!("{base}-ultrafast")),
        (base.to_string(), Some(CodexServiceTier::Ultrafast))
    );
    assert_eq!(
        parse_service_tier_model_alias(base),
        (base.to_string(), None)
    );
}
