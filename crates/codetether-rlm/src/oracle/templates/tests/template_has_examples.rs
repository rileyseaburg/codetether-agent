//! Catalog regression.

use super::*;

#[test]
fn template_has_examples() {
    for template in QueryTemplate::all() {
        assert!(
            !template.example.is_empty(),
            "Template {} has no examples",
            template.id
        );
    }
}
