//! Payload and log protocol for remotely executed swarm tasks.
//!
//! [RemoteSubtaskPayload] carries task configuration; [encode_payload] and
//! [decode_payload] preserve the Base64/JSON wire representation. Extract probe
//! and result records from logs, or construct a fallback through [final_result].
use crate::SubTaskResult;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
mod codec;
mod constants;
mod logs;
mod payload;
mod probe;
mod result;
pub use codec::{decode_payload, encode_payload};
pub use constants::{
    SWARM_SUBTASK_PAYLOAD_ENV, SWARM_SUBTASK_PROBE_PREFIX, SWARM_SUBTASK_RESULT_PREFIX,
};
pub use logs::{latest_probe_from_logs, probe_changed_files_set, result_from_logs};
pub use payload::RemoteSubtaskPayload;
pub use probe::RemoteBranchProbe;
pub use result::final_result;
#[cfg(test)]
mod tests;

mod agent_result;
mod agent_success;
pub use agent_result::from_agent_result;

/// Prompt assembly for remote task payloads.
pub mod prompts;
