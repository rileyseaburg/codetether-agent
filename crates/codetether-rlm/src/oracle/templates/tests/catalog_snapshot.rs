//! Complete template catalog and rendering contract.
use super::*;
#[test]
fn complete_catalog_snapshot() {
    let rows: Vec<_> = QueryTemplate::all()
        .iter()
        .map(|t| {
            serde_json::json!({
        "id":t.id,"description":t.description,"template":t.template,"kind":t.kind,
        "params":t.params,"example":t.example,"rendered":t.render(t.example),
        "generated":format!("{:?}",GeneratedQuery::render(t,t.example))})
        })
        .collect();
    println!("CATALOG={}", serde_json::to_string(&rows).unwrap());
    for kind in [
        TemplateKind::Grep,
        TemplateKind::Ast,
        TemplateKind::Semantic,
    ] {
        let expected: Vec<_> = QueryTemplate::all()
            .iter()
            .filter(|t| t.kind == kind)
            .map(|t| t.id)
            .collect();
        assert_eq!(
            QueryTemplate::iter_kind(kind)
                .map(|t| t.id)
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            QueryTemplate::by_kind(kind)
                .map(|t| t.id)
                .collect::<Vec<_>>(),
            expected
        );
    }
    assert_eq!(QueryTemplate::iter().count(), QueryTemplate::all().len());
    let first = QueryTemplate::find("grep_find_occurrences").unwrap();
    assert_eq!(
        first.render(&["fn"]),
        "Find all occurrences of fn in {FILE}"
    );
    assert_eq!(GeneratedQuery::render(first, &[]).file, "");
}
