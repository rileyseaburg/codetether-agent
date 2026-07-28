//! Whitespace-normalized text comparisons for locators.

pub(crate) fn clean(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn contains(actual: &str, expected: &str) -> bool {
    let actual = clean(actual);
    let expected = clean(expected);
    !expected.is_empty() && actual.contains(&expected)
}

pub(crate) fn exact(actual: &str, expected: &str) -> bool {
    let expected = clean(expected);
    !expected.is_empty() && clean(actual) == expected
}
