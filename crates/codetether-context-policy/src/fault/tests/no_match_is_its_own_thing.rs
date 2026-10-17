//! Fault contract regression.
use crate::fault::Fault;

#[test]
fn no_match_is_its_own_thing() {
    let f = Fault::NoMatch;
    assert!(f.is_no_match());
    assert!(!f.is_denied());
    assert_eq!(f.code(), "no_match");
    assert_eq!(f.to_string(), "no match");
}
