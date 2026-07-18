use super::ForkTurns;

#[test]
fn defaults_to_all() {
    assert_eq!(ForkTurns::parse(None).unwrap(), ForkTurns::All);
}

#[test]
fn accepts_named_and_count_selectors() {
    assert_eq!(ForkTurns::parse(Some("none")).unwrap(), ForkTurns::None);
    assert_eq!(ForkTurns::parse(Some("all")).unwrap(), ForkTurns::All);
    assert_eq!(ForkTurns::parse(Some("3")).unwrap(), ForkTurns::Count(3));
}

#[test]
fn rejects_zero_and_unknown_values() {
    assert!(ForkTurns::parse(Some("0")).is_err());
    assert!(ForkTurns::parse(Some("recent")).is_err());
}
