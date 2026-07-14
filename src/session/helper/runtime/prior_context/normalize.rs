//! Text normalization for prior-context directives.

/// Split and normalize user text into independently classifiable clauses.
pub(super) fn clauses(input: &str) -> Vec<String> {
    let mut example_block = false;
    super::quoted::strip(input)
        .to_lowercase()
        .lines()
        .flat_map(|line| {
            if line.trim().is_empty() {
                example_block = false;
            }
            let starts_block = super::example_line::starts_block(line);
            let example = example_block || starts_block || super::example_line::matches(line);
            example_block |= starts_block;
            line_clauses(line, example)
        })
        .collect()
}

fn line_clauses(input: &str, example: bool) -> Vec<String> {
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
