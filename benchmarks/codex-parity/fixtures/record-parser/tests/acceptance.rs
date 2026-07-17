use record_parser::{ParseError, Record, parse_record};

#[test]
fn parses_trimmed_record() {
    assert_eq!(
        parse_record(" alice = 42 "),
        Ok(Record {
            name: "alice".into(),
            score: 42
        })
    );
}

#[test]
fn returns_each_typed_error_without_panicking() {
    assert_eq!(parse_record("alice"), Err(ParseError::MissingSeparator));
    assert_eq!(parse_record(" = 1"), Err(ParseError::EmptyName));
    assert_eq!(parse_record("alice = nope"), Err(ParseError::InvalidScore));
}
