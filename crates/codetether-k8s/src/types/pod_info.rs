//! PodInfo data contract.

use serde::{Deserialize, Serialize};

/// Summary information about a pod.
///
/// # Examples
/// ```
/// use codetether_k8s::{PodInfo};
/// let pod = PodInfo { name: "agent".into(), phase: "Running".into(),
///     ready: true, start_time: None };
/// assert!(pod.ready);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodInfo {
    pub name: String,
    pub phase: String,
    pub ready: bool,
    pub start_time: Option<String>,
}
