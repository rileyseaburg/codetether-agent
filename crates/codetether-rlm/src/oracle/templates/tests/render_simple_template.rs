//! Catalog regression.

use super::*;

#[test]
fn render_simple_template() {
    let template = QueryTemplate::find("grep_find_occurrences").unwrap();
    let rendered = template.render(&["fn", "src/main.rs"]);
    assert_eq!(rendered, "Find all occurrences of fn in src/main.rs");
}
