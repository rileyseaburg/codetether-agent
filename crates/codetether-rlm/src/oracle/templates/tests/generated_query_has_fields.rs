//! Catalog regression.

use super::*;

#[test]
fn generated_query_has_fields() {
    let template = QueryTemplate::find("grep_find_occurrences").unwrap();
    let r#gen = GeneratedQuery::render(template, &["async fn", "src/lib.rs"]);

    assert_eq!(
        r#gen.query,
        "Find all occurrences of async fn in src/lib.rs"
    );
    assert_eq!(r#gen.template_id, "grep_find_occurrences");
    assert_eq!(r#gen.kind, TemplateKind::Grep);
    assert_eq!(r#gen.file, "src/lib.rs");
}
