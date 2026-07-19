//! Client access to the authoritative coordinator inherited from `mux new`.

#[path = "coordination/client.rs"]
mod client;
#[path = "coordination/event.rs"]
mod event;
#[path = "coordination/operation.rs"]
mod operation;

use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) const SESSION_ENV: &str = "CODETETHER_MUX_SESSION";

pub(crate) async fn acquire(
    owner: &str,
    agent: &str,
    workspace: &Path,
    paths: Vec<PathBuf>,
) -> Result<Option<crate::mux::lease::CoordinationReply>> {
    operation::acquire(owner, agent, workspace, paths).await
}

pub(crate) async fn renew(owner: &str) -> Result<Option<crate::mux::lease::CoordinationReply>> {
    operation::send(
        "renew",
        owner,
        crate::mux::lease::CoordinationRequest::Renew {
            owner: owner.into(),
        },
    )
    .await
}

pub(crate) async fn release(owner: &str) -> Result<Option<crate::mux::lease::CoordinationReply>> {
    operation::send(
        "release",
        owner,
        crate::mux::lease::CoordinationRequest::Release {
            owner: owner.into(),
        },
    )
    .await
}

pub(crate) fn active() -> bool {
    std::env::var_os(SESSION_ENV).is_some()
}
