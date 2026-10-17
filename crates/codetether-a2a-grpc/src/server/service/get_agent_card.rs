//! get agent card RPC operation.
use super::*;
pub(super) async fn get_agent_card(
    service: &A2aServiceImpl,
    _request: Request<proto::GetAgentCardRequest>,
) -> Result<Response<proto::AgentCard>, Status> {
    Ok(Response::new(bridge::local_card_to_proto(
        &service.store.card,
    )))
}
