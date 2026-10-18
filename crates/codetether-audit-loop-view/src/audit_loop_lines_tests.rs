//! Test for the audit-loop line builder.

use super::super::audit_loop_lines::build_lines;

#[test]
fn build_lines_includes_task_and_gate() {
    let s = super::run();
    let text: String = build_lines(&s)
        .iter()
        .flat_map(|l| l.spans.iter().map(|sp| sp.content.to_string()))
        .collect();
    assert!(text.contains("tables") && text.contains("test"));
}
