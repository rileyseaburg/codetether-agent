//! Normalize and classify individual source-text lines.

pub(super) fn line_clauses(input: &str, example: bool) -> Vec<String> {
    input
        .split(['.', ';', '!', '?'])
        .flat_map(split_contrasts)
        .map(normalize)
        .filter(|clause| !clause.is_empty())
        .map(|clause| {
            if example {
                format!("example {clause}")
            } else {
                clause
            }
        })
        .collect()
}

fn split_contrasts(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(" but ")
        .flat_map(|part| part.split(" instead "))
}

fn normalize(input: &str) -> String {
    input
        .replace(['\'', '’'], "")
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
