//! Render final-answer notices for unclassified deliverables.
use codetether_evidence_core::gate_mode::GateMode;

/// Append a ledger notice for unclassified deliverables.
/// # Arguments
/// `mode` selects notice wording; `answer` is the response; `session_id` identifies the ledger; `missing` is the unclassified count.
/// # Returns
/// The original answer in Off mode, otherwise the answer with its ledger notice.
/// # Examples
/// ```rust
/// use codetether_evidence_core::gate_mode::GateMode;
/// assert_eq!(codetether_evidence_ledger::final_note::render(GateMode::Off, "ok", "s", 0), "ok");
/// ```
pub fn render(mode: GateMode, answer: &str, session_id: &str, missing: usize) -> String {
    let path = super::ledger_path::for_session(session_id);
    let prefix = match mode {
        GateMode::Strict => "Runtime scope gate STRICT",
        GateMode::Warn => "Runtime scope ledger",
        GateMode::Off => return answer.to_string(),
    };
    format!(
        "{answer}\n\n{prefix}: {missing} deliverable(s) were not explicitly classified as proven, pending, blocked, or not-run. Ledger: {}",
        path.display()
    )
}
