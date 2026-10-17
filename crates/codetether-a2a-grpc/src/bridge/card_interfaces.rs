//! Agent-card additional_interfaces projection.
use super::*;
pub(super) fn interfaces(card: &local::AgentCard) -> Vec<proto::AgentInterface> {
    card.additional_interfaces
        .iter()
        .map(|i| proto::AgentInterface {
            url: i.url.clone(),
            transport: i.transport.clone(),
        })
        .collect()
}
