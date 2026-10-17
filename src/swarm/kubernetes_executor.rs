//! Compatibility exports for the shared remote swarm wire protocol.
pub use codetether_swarm_core::remote_protocol::{
    RemoteBranchProbe, RemoteSubtaskPayload, SWARM_SUBTASK_PAYLOAD_ENV, SWARM_SUBTASK_PROBE_PREFIX,
    SWARM_SUBTASK_RESULT_PREFIX, decode_payload, encode_payload, latest_probe_from_logs,
    probe_changed_files_set, result_from_logs,
};
