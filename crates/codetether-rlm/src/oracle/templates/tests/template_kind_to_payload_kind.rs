//! Catalog regression.

use super::*;

#[test]
fn template_kind_to_payload_kind() {
    assert_eq!(TemplateKind::Grep.as_payload_kind(), "grep");
    assert_eq!(TemplateKind::Ast.as_payload_kind(), "ast");
    assert_eq!(TemplateKind::Semantic.as_payload_kind(), "semantic");
}
