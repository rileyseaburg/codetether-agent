use super::config::parse;

#[test]
fn invalid_values_fall_back_and_valid_values_are_clamped() {
    assert_eq!(parse(None, 4, 1, 32), 4);
    assert_eq!(parse(Some("invalid"), 4, 1, 32), 4);
    assert_eq!(parse(Some("0"), 4, 1, 32), 1);
    assert_eq!(parse(Some("99"), 4, 1, 32), 32);
}
