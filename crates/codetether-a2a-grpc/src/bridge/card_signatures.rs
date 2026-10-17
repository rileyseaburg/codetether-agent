//! Agent-card signatures projection.
use super::*;
pub(super) fn signatures(card: &local::AgentCard) -> Vec<proto::AgentCardSignature> {
    card.signatures
        .iter()
        .map(|s| proto::AgentCardSignature {
            protected: s.algorithm.clone().unwrap_or_default(),
            signature: s.signature.clone(),
            header: None,
        })
        .collect()
}
