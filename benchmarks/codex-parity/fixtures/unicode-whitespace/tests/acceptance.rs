use unicode_whitespace::count_words;

#[test]
fn handles_unicode_and_repeated_whitespace() {
    assert_eq!(count_words(" alpha\tβeta\n\u{2003}gamma  "), 3);
}

#[test]
fn handles_empty_input() {
    assert_eq!(count_words(" \t\n"), 0);
}
