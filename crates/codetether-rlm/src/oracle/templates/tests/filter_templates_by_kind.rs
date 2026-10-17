//! Catalog regression.

use super::*;

#[test]
fn filter_templates_by_kind() {
    let grep_count = QueryTemplate::iter_kind(TemplateKind::Grep).count();
    let ast_count = QueryTemplate::iter_kind(TemplateKind::Ast).count();

    assert!(grep_count > 0);
    assert!(ast_count > 0);
}
