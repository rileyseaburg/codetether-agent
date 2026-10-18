//! Extract the semantic answer from RLM result envelopes.
use codetether_rlm::FinalPayload;

pub(crate) fn extract_semantic_handoff_from_rlm(answer: &str) -> String {
    match FinalPayload::parse(answer) {
        FinalPayload::Semantic(payload) => payload.answer,
        _ => answer.trim().to_string(),
    }
}
