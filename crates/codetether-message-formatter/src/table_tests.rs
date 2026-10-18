//! Formatter integration with shared Markdown tables.

#[test]
fn format_content_renders_a_table() {
    let fmt = super::super::MessageFormatter::new(80);
    let md = "| File | Change |\n|------|--------|\n| a.rs | edited |";
    let lines = fmt.format_content(md, "assistant");
    let joined: String = lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
        .collect();
    assert!(joined.contains('─') && joined.contains("File") && joined.contains("a.rs"));
}
