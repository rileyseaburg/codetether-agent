//! SubagentPodState data contract.

use serde::{Deserialize, Serialize};

/// Summary of a running/completed sub-agent pod.
///
/// # Examples
/// ```
/// use codetether_k8s::{SubagentPodState};
/// let state = SubagentPodState { pod_name: "agent".into(), phase: "Pending".into(),
///     ready: false, terminated: false, exit_code: None, reason: None, restart_count: 0 };
/// assert!(!state.terminated);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentPodState {
    pub pod_name: String,
    pub phase: String,
    pub ready: bool,
    pub terminated: bool,
    pub exit_code: Option<i32>,
    pub reason: Option<String>,
    pub restart_count: u32,
}
