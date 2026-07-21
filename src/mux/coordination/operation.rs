//! Typed coordination requests over an authenticated mux connection.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

use crate::mux::lease::{CoordinationReply, CoordinationRequest};
use crate::mux::protocol::{ClientRequest, ServerResponse};

pub(super) async fn acquire(
    owner: &str,
    agent: &str,
    workspace: &Path,
    paths: Vec<PathBuf>,
) -> Result<Option<CoordinationReply>> {
    let request = CoordinationRequest::Acquire {
        owner: owner.into(),
        agent: agent.into(),
        workspace: workspace.into(),
        paths,
        wait_ms: crate::mux::lease::ACQUIRE_WAIT_MILLIS,
    };
    send("acquire", owner, request).await
}

pub(super) async fn send(
    action: &str,
    owner: &str,
    request: CoordinationRequest,
) -> Result<Option<CoordinationReply>> {
    let Some(mut connection) = super::client::connect().await? else {
        return Ok(None);
    };
    let response = connection
        .request(ClientRequest::Coordinate { request })
        .await?;
    let reply = match response {
        ServerResponse::Coordination { reply } => reply,
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux returned an invalid coordination response"),
    };
    super::event::publish(action, owner, &reply);
    Ok(Some(reply))
}
