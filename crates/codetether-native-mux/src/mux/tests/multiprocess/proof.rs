//! Durable, token-free multiprocess proof artifact.

use std::path::{Path, PathBuf};

use crate::mux::model::MuxSnapshot;
use crate::mux::registry::MuxRecord;

pub(super) async fn write(
    root: &Path,
    alpha: &MuxRecord,
    beta: &MuxRecord,
    alpha_state: &MuxSnapshot,
    beta_state: &MuxSnapshot,
) -> PathBuf {
    let artifact = root.join("mux-multiprocess-proof.json");
    let value = serde_json::json!({
        "verified": true,
        "alpha": { "pid": alpha.pid, "address": alpha.address, "state": alpha_state },
        "beta": { "pid": beta.pid, "address": beta.address, "state": beta_state },
    });
    tokio::fs::write(&artifact, serde_json::to_vec_pretty(&value).unwrap())
        .await
        .unwrap();
    artifact
}
