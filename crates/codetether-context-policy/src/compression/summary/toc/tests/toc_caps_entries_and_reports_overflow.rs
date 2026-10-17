//! toc caps entries and reports overflow.
use super::*;
#[test]
fn toc_caps_entries_and_reports_overflow() {
    let prefix: Vec<Message> = (0..15)
        .map(|i| text(Role::User, &format!("request number {i}")))
        .collect();
    let toc = render_toc(&prefix, 0);
    assert!(toc.contains("plus 5 more user turns"));
    assert!(!toc.contains("request number 12"));
}
