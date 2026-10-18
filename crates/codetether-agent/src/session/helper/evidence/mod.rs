//! Native session adapter for shared evidence runtime services.
pub(crate) fn gate_final_answer(answer: &str, session: &crate::session::Session) -> String {
    codetether_evidence_runtime::gate_final_answer(answer, &session.id, &session.messages)
}
