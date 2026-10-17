//! toc truncates long first lines.
use super::*;
#[test]
fn toc_truncates_long_first_lines() {
    let long = "x".repeat(200);
    let toc = render_toc(&[text(Role::User, &long)], 0);
    assert!(toc.contains('…'));
}
