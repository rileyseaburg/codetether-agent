//! Select the proof contract implied by task text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The proof contract inferred from a task's platform and deployment markers.
/// # Examples
/// ```
/// let workflow = codetether_evidence_core::workflow::infer("upload to YouTube");
/// assert!(workflow.required_gate().contains("platform ID"));
/// ```
pub enum ProofWorkflow {
    /// Artifact and concrete platform upload identity.
    PlatformUpload,
    /// Live deployment identity and successful state.
    LiveDeployment,
    /// Local command, exit state and any produced artifacts.
    LocalOnly,
}

impl ProofWorkflow {
    /// Describe the evidence required by this workflow category.
    pub fn required_gate(self) -> &'static str {
        match self {
            Self::PlatformUpload => "artifact plus concrete platform ID",
            Self::LiveDeployment => "live app/job/pod identity plus success state",
            Self::LocalOnly => "command, exit state, and preserved artifact path if produced",
        }
    }
}

/// Infer a proof workflow using the established marker precedence.
/// # Arguments
/// `text` describes the task or requested proof.
/// # Returns
/// Platform upload, live deployment or local-only proof requirements.
/// # Examples
/// ```
/// use codetether_evidence_core::workflow::{infer, ProofWorkflow};
/// assert_eq!(infer("review code"), ProofWorkflow::LocalOnly);
/// ```
pub fn infer(text: &str) -> ProofWorkflow {
    let lower = text.to_ascii_lowercase();
    if lower.contains("youtube") || lower.contains("tiktok") || lower.contains("upload") {
        ProofWorkflow::PlatformUpload
    } else if lower.contains("argo") || lower.contains("kubectl") || lower.contains("cluster") {
        ProofWorkflow::LiveDeployment
    } else {
        ProofWorkflow::LocalOnly
    }
}
