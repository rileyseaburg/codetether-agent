use super::*;

#[test]
fn parses_mention() {
    let (name, msg) = parse_mention("@forge fix the tests").unwrap();
    assert_eq!(name, "forge");
    assert_eq!(msg, "fix the tests");
}

#[test]
fn no_mention() {
    assert!(parse_mention("regular message").is_none());
}

#[test]
fn empty_message() {
    assert!(parse_mention("@forge").is_none());
}
