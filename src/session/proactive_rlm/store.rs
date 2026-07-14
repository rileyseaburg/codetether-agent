//! Atomic persistence for prepared proactive-RLM artifacts.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::session::Session;
use crate::session::index::SummaryIndex;

const SCHEMA_VERSION: u8 = 1;

#[derive(Serialize, Deserialize)]
pub(super) struct Prepared {
    pub schema_version: u8,
    /// Length of the transcript prefix covered when this index was built.
    pub message_count: usize,
    /// Fingerprint of that prefix; later appended messages remain compatible.
    pub fingerprint: u64,
    pub generation: u64,
    pub index: SummaryIndex,
}

pub(super) async fn read(session: &Session) -> Option<Prepared> {
    read_id(&session.id).await
}

pub(super) async fn read_id(session_id: &str) -> Option<Prepared> {
    let path = path(session_id).ok()?;
    let bytes = tokio::fs::read(path).await.ok()?;
    decode(&bytes)
}

pub(super) fn decode(bytes: &[u8]) -> Option<Prepared> {
    let prepared: Prepared = serde_json::from_slice(bytes).ok()?;
    (prepared.schema_version == SCHEMA_VERSION).then_some(prepared)
}

pub(super) async fn remove(session_id: &str) {
    if let Ok(path) = path(session_id) {
        let _ = tokio::fs::remove_file(path).await;
    }
}

pub(super) async fn write(session_id: &str, value: &Prepared) -> Result<()> {
    let path = path(session_id)?;
    let tmp = path.with_extension("rlm.json.tmp");
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(&tmp, serde_json::to_vec(value)?).await?;
    tokio::fs::rename(tmp, path).await?;
    Ok(())
}

fn path(session_id: &str) -> Result<std::path::PathBuf> {
    Session::session_path(session_id)?;
    Ok(Session::sessions_dir()?
        .join("rlm")
        .join(format!("{session_id}.json")))
}
