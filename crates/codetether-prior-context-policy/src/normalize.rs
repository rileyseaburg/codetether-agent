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
            super::normalize_line::line_clauses(line, example)
        })
        .collect()
}
