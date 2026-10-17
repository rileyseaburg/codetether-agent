//! Catalog regression.

use super::*;

#[test]
fn find_template_by_id() {
    let template = QueryTemplate::find("ast_list_functions").unwrap();
    assert_eq!(template.kind, TemplateKind::Ast);
}
