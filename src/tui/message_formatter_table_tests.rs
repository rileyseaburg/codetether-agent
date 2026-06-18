//! Tests for markdown table detection and rendering.

use super::*;

#[test]
fn detects_separator_and_rows() {
    assert!(is_separator_row("|---|---|"));
    assert!(is_separator_row("| :--- | ---: | :--: |"));
    assert!(!is_separator_row("| a | b |"));
    assert!(is_table_row("| a | b |"));
    assert!(!is_table_row("no pipe here"));
}

#[test]
fn splits_cells_trimming_pipes() {
    assert_eq!(parse::split_cells("| a | b |"), vec!["a", "b"]);
    assert_eq!(parse::split_cells("a|b|c"), vec!["a", "b", "c"]);
}

#[test]
fn renders_borderless_with_header_rule() {
    let buf = vec![
        "| File | Change |".to_string(),
        "|------|--------|".to_string(),
        "| a.rs | edited |".to_string(),
    ];
    let out = render_table(&buf, 80).expect("valid table");
    // header + rule + 1 body row = 3 lines (no top/bottom border)
    assert_eq!(out.len(), 3);
    let header_bold = out[0].spans.iter().any(|s| {
        s.style
            .add_modifier
            .contains(ratatui::style::Modifier::BOLD)
    });
    assert!(header_bold);
    let rule: String = out[1].spans.iter().map(|s| s.content.as_ref()).collect();
    assert!(rule.contains('─') && !rule.contains('┼'));
}

#[test]
fn rejects_without_separator() {
    let buf = vec!["| a | b |".to_string(), "| c | d |".to_string()];
    assert!(render_table(&buf, 80).is_none());
}

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
