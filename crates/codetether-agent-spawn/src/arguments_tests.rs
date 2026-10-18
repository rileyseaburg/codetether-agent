//! Existing spawn argument regression coverage.
use super::parse_spawn_args as parse;

#[test]
fn parses_supported_name_and_instructions() {
    let parsed = parse("reviewer audit the PR").expect("valid arguments");

    assert_eq!(parsed.name, "reviewer");
    assert_eq!(parsed.instructions, "audit the PR");
    assert!(parsed.parent.is_none());
}

#[test]
fn exposes_unsupported_parent_anywhere_for_rejection() {
    let parsed = parse("reviewer audit --parent build carefully").expect("parsed arguments");

    assert_eq!(parsed.parent.as_deref(), Some("build"));
    assert_eq!(parsed.instructions, "audit carefully");
}

#[test]
fn exposes_incomplete_parent_for_rejection() {
    let parsed = parse("reviewer audit --parent").expect("parsed arguments");

    assert_eq!(parsed.parent.as_deref(), Some(""));
    assert_eq!(parsed.instructions, "audit");
}
