//! Mapping authenticated coordination requests onto the mux lease table.

use super::context::ServerContext;
use crate::mux::lease::CoordinationRequest;
use crate::mux::protocol::ServerResponse;

pub(super) async fn apply(context: &ServerContext, request: CoordinationRequest) -> ServerResponse {
    let result = match request {
        CoordinationRequest::Acquire {
            owner,
            agent,
            workspace,
            paths,
            wait_ms,
        } => acquire(context, &owner, &agent, &workspace, paths, wait_ms).await,
        CoordinationRequest::Renew { owner } => {
            super::coordination_identity::valid(&owner).map(|_| context.leases.renew(&owner))
        }
        CoordinationRequest::Release { owner } => {
            super::coordination_identity::valid(&owner).map(|_| context.leases.release(&owner))
        }
        CoordinationRequest::Snapshot => Ok(context.leases.snapshot()),
    };
    match result {
        Ok(reply) => ServerResponse::Coordination { reply },
        Err(error) => ServerResponse::Error {
            message: error.to_string(),
        },
    }
}

async fn acquire(
    context: &ServerContext,
    owner: &str,
    agent: &str,
    requested: &std::path::Path,
    paths: Vec<std::path::PathBuf>,
    wait_ms: u64,
) -> anyhow::Result<crate::mux::lease::CoordinationReply> {
    super::coordination_identity::valid(owner)?;
    super::coordination_identity::valid(agent)?;
    let workspace = super::coordination_path::workspace(requested)?;
    let relative = super::coordination_path::relative_all(&workspace, paths)?;
    Ok(context
        .leases
        .acquire_wait(owner, agent, &workspace, relative, wait_ms)
        .await)
}
