//! Fresh source-metric requirements for delegated refactors.

pub(super) fn render(specialty: &str, instruction: &str, read_only: bool) -> &'static str {
    if read_only || !is_refactor(specialty, instruction) {
        return "";
    }
    "SOURCE METRIC CONTRACT:\n- Immediately before planning or editing, inspect every target file in the current workspace and record a fresh baseline.\n- Report physical line count and code-line count excluding comments and blank lines, naming the command or tool used.\n- After edits, rerun the same measurement and report per-file before/after counts.\n- Treat remembered, approximate, or prior-session counts as context only, never as current evidence.\n- If target files are not named, locate them from the current workspace before measuring."
}

fn is_refactor(specialty: &str, instruction: &str) -> bool {
    specialty.eq_ignore_ascii_case("refactor")
        || instruction.to_ascii_lowercase().contains("refactor")
}

#[cfg(test)]
#[path = "source_metric_contract_tests.rs"]
mod tests;
