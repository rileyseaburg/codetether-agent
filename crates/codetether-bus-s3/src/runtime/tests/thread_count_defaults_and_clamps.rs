//! Existing bounded executor regression.
use super::*;

#[test]
fn thread_count_defaults_and_clamps() {
    assert_eq!(parse(None), 2);
    assert_eq!(parse(Some("0")), 1);
    assert_eq!(parse(Some("99")), 8);
    assert_eq!(parse(Some("invalid")), 2);
}
