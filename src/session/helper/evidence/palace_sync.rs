//! Persist proven ledger items as project-palace beliefs.
//!
//! Closes the loop between the evidence writeback path and
//! [`crate::memory::palace`]: proven deliverables become durable
//! `.codetether/memory.json` beliefs injected into future prompts.

use crate::memory::palace;

use super::ledger::ScopeLedger;

/// Sync proven ledger items into the project memory palace.
///
/// No-ops when disabled via `CODETETHER_PALACE_WRITEBACK=off`, when the
/// ledger has no proven items, or when the cwd is unavailable.
pub(crate) fn save(ledger: &ScopeLedger) {
    if disabled() {
        return;
    }
    let Ok(root) = std::env::current_dir() else {
        return;
    };
    let proven: Vec<_> = ledger
        .items
        .iter()
        .filter(|item| item.status == "proven")
        .collect();
    if proven.is_empty() {
        return;
    }
    let mut beliefs = palace::load_project_beliefs(&root);
    for item in proven {
        let key = super::palace_belief::belief_key(&item.deliverable);
        if key.is_empty() {
            continue;
        }
        match beliefs.iter_mut().find(|b| b.belief_key == key) {
            Some(existing) => existing.revalidation_success(),
            None => beliefs.push(super::palace_belief::from_item(item, &ledger.session_id)),
        }
    }
    if let Err(err) = palace::save_project_beliefs(&root, &beliefs) {
        tracing::warn!(error = %err, "Failed to persist project palace beliefs");
    }
}

fn disabled() -> bool {
    matches!(
        std::env::var("CODETETHER_PALACE_WRITEBACK").ok().as_deref(),
        Some("off")
    )
}
