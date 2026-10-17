//! Provenance ledger for delegated execution constraints.

use super::constraint_entry::ConstraintEntry;
pub(crate) use super::constraint_mode::Mode;

pub(super) fn render(
    instruction: &str,
    context: &str,
    repository_policy: &str,
    mode: Mode,
) -> String {
    let mut entries =
        super::constraint_extract::from_text("delegated task instruction", instruction);
    entries.extend(super::constraint_extract::from_text(
        "prior dependency context",
        context,
    ));
    entries.extend(super::constraint_extract::from_text(
        "repository policy (AGENTS.md)",
        repository_policy,
    ));
    entries.extend(mode.entry());
    render_entries(&entries)
}

fn render_entries(entries: &[ConstraintEntry]) -> String {
    if entries.is_empty() {
        return "TASK CONSTRAINT LEDGER:\n- No execution prohibitions were sourced. Do not invent test, build, compiler, linter, or watcher bans; run focused verification as appropriate.".into();
    }
    let rows = entries
        .iter()
        .map(|entry| format!("- [{}] {}", entry.source, entry.text))
        .collect::<Vec<_>>()
        .join("\n");
    format!("TASK CONSTRAINT LEDGER:\n{rows}\nEnforce only sourced prohibitions listed above.")
}

#[cfg(test)]
#[path = "constraint_ledger_tests.rs"]
mod tests;
