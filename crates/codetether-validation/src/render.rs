//! Render one bounded diagnostic continuation report.
use crate::ValidationReport;
use std::collections::BTreeMap;

const MAX_DIAGNOSTICS_PER_FILE: usize = 25;
pub(super) fn report(
    issues_by_file: BTreeMap<String, Vec<String>>,
    issue_count: usize,
) -> Option<ValidationReport> {
    if issues_by_file.is_empty() {
        return None;
    }

    let mut prompt = String::from(
        "Mandatory post-edit verification found unresolved diagnostics in files you changed. \
Do not finish yet. Fix every issue below, respecting workspace config files such as eslint, biome, \
ruff, stylelint, tsconfig, and project-local language-server settings. Prefer direct file-edit \
tools on the listed files. Do not wander into unrelated files or exploratory bash loops unless a \
minimal validation command is strictly necessary. After fixing them, re-check the same files and \
only then provide the final answer.\n\n",
    );

    for (path, diagnostics) in issues_by_file {
        prompt.push_str(&format!("{path}\n"));
        for diagnostic in diagnostics.iter().take(MAX_DIAGNOSTICS_PER_FILE) {
            prompt.push_str("  - ");
            prompt.push_str(diagnostic);
            prompt.push('\n');
        }
        if diagnostics.len() > MAX_DIAGNOSTICS_PER_FILE {
            prompt.push_str(&format!(
                "  - ... {} more diagnostics omitted\n",
                diagnostics.len() - MAX_DIAGNOSTICS_PER_FILE
            ));
        }
        prompt.push('\n');
    }

    Some(ValidationReport {
        issue_count,
        prompt: prompt.trim_end().to_string(),
    })
}
