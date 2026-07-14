#[test]
fn generated_tokens_are_random_hex() {
    let first = crate::mux::token::generate();
    let second = crate::mux::token::generate();
    assert_eq!(first.len(), 64);
    assert!(first.chars().all(|ch| ch.is_ascii_hexdigit()));
    assert_ne!(first, second);
    assert!(crate::mux::token::matches(&first, &first));
    assert!(!crate::mux::token::matches(&first, &second));
}

#[test]
fn registry_names_reject_path_traversal() {
    assert!(crate::mux::registry::validate_name("work_1").is_ok());
    assert!(crate::mux::registry::validate_name("../work").is_err());
}
