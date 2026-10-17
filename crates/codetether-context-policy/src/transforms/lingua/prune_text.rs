//! prune text.
use super::*;

pub(super) fn prune_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_code = false;
    let mut blank_run = 0usize;

    for line in input.split_inclusive('\n') {
        let stripped_nl = line.trim_end_matches('\n');
        let fence = stripped_nl.trim_start().starts_with("```");
        if fence {
            in_code = !in_code;
            out.push_str(line);
            blank_run = 0;
            continue;
        }
        if in_code {
            out.push_str(line);
            continue;
        }

        // Trim trailing whitespace.
        let trimmed = stripped_nl.trim_end();

        // Drop pure-punctuation or ellipsis-only lines.
        let is_noise = trimmed.is_empty()
            || trimmed.chars().all(|c| c.is_ascii_whitespace())
            || trimmed
                .chars()
                .all(|c| matches!(c, '.' | '…' | '-' | '_' | '*' | '='));

        if is_noise {
            blank_run += 1;
            if blank_run <= 1 {
                out.push('\n');
            }
            continue;
        }
        blank_run = 0;

        // Collapse internal runs of spaces.
        let collapsed = collapse_spaces(trimmed);
        out.push_str(&collapsed);
        if line.ends_with('\n') {
            out.push('\n');
        }
    }

    out
}
