use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serializable branch-health observation emitted by a remote worker.
///
/// # Examples
/// ```
/// let probe = codetether_swarm_core::remote_protocol::RemoteBranchProbe { subtask_id: "task".into(), compile_ok: true, changed_files: vec![], changed_lines: 0 };
/// assert!(probe.compile_ok);
/// ```
pub struct RemoteBranchProbe {
    pub subtask_id: String,
    pub compile_ok: bool,
    pub changed_files: Vec<String>,
    pub changed_lines: u32,
}
