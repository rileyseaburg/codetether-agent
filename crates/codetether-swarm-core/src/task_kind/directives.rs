//! Preserve clause boundaries when recognizing task directives.

pub(super) fn tokens(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    for clause in text.split([',', ';', '.', '\n', '!', '?']) {
        result.push(String::new());
        result.extend(super::tokens(clause));
    }
    result
}
