use super::*;

/// Sub-agent execution mode.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
///
/// # Examples
/// ```
/// use codetether_swarm_core::ExecutionMode;
/// let mode = ExecutionMode::from_cli_value("k8s");
/// match mode {
///     ExecutionMode::LocalThread => panic!("expected Kubernetes"),
///     ExecutionMode::KubernetesPod => {}
/// }
/// ```
pub enum ExecutionMode {
    /// Run sub-agents as local async tasks in the current process.
    #[default]
    LocalThread,
    /// Run sub-agents as isolated Kubernetes pods.
    KubernetesPod,
}

impl ExecutionMode {
    /// Parse the accepted CLI aliases, defaulting unknown values to local execution.
    ///
    /// # Arguments
    /// * `value` - Requested CLI mode (for example `k8s` or `pod`).
    ///
    /// # Returns
    /// KubernetesPod for a recognized remote alias, otherwise LocalThread.
    ///
    /// # Examples
    /// ```
    /// use codetether_swarm_core::ExecutionMode;
    /// assert_eq!(ExecutionMode::from_cli_value("pod"), ExecutionMode::KubernetesPod);
    /// ```
    pub fn from_cli_value(value: &str) -> Self {
        match value {
            "k8s" | "kubernetes" | "kubernetes-pod" | "pod" => Self::KubernetesPod,
            _ => Self::LocalThread,
        }
    }
}
