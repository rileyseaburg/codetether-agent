//! Apply evidence checks and persist scope evidence before final response.
use codetether_provider_core::Message;

/// Apply the configured final-answer gate and persist its scope evidence.
/// # Arguments
/// `answer` is the proposed final response; `session_id` and `messages` identify
/// the session and supply the scope/evidence content.
/// # Returns
/// The original, warned or rewritten answer with any missing-scope notice.
/// Persistence and palace synchronization preserve their best-effort semantics.
/// # Examples
/// ```no_run
/// let _answer = codetether_evidence_runtime::gate_final_answer("Reviewed.", "s", &[]);
/// ```
pub fn gate(answer: &str, session_id: &str, messages: &[Message]) -> String {
    let mode = super::gate_mode::current();
    if mode == super::gate_mode::GateMode::Off {
        return answer.to_string();
    }
    let answer = super::verification_gate::apply(mode, answer);
    let ledger = super::ledger::build(session_id, messages);
    super::ledger_persist::save(&ledger);
    super::writeback_persist::save(session_id, &ledger);
    super::palace_sync::save(&ledger);
    if ledger.items.is_empty() {
        return answer;
    }
    let missing = super::ledger::unclassified_count(&ledger, &answer);
    if missing == 0 {
        return answer;
    }
    super::final_note::render(mode, &answer, session_id, missing)
}
