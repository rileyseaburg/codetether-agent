//! Authenticated mux server regression tests.

#[test]
fn generated_tokens_are_random_hex() {
    let first = crate::token::generate();
    let second = crate::token::generate();
    assert_eq!(first.len(), 64);
    assert!(first.chars().all(|ch| ch.is_ascii_hexdigit()));
    assert_ne!(first, second);
    assert!(crate::token::matches(&first, &first));
    assert!(!crate::token::matches(&first, &second));
}

#[test]
fn registry_names_reject_path_traversal() {
    assert!(codetether_mux_registry::validate_name("work_1").is_ok());
    assert!(codetether_mux_registry::validate_name("../work").is_err());
}
