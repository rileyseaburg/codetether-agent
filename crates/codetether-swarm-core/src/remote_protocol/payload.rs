use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serializable task configuration sent to a remote worker.
///
/// # Examples
/// ```
/// # fn example(payload: codetether_swarm_core::remote_protocol::RemoteSubtaskPayload) {
/// let encoded = codetether_swarm_core::remote_protocol::encode_payload(&payload).unwrap();
/// let decoded = codetether_swarm_core::remote_protocol::decode_payload(&encoded).unwrap();
/// assert_eq!(decoded.subtask_id, payload.subtask_id);
/// # }
/// ```
pub struct RemoteSubtaskPayload {
    pub swarm_id: String,
    pub subtask_id: String,
    pub subtask_name: String,
    pub specialty: String,
    pub instruction: String,
    pub context: String,
    pub provider: String,
    pub model: String,
    pub max_steps: usize,
    pub timeout_secs: u64,
    pub working_dir: Option<String>,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub verification: bool,
    #[serde(default)]
    pub probe_interval_secs: u64,
}
