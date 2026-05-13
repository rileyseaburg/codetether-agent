#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProofWorkflow {
    PlatformUpload,
    LiveDeployment,
    LocalOnly,
}

impl ProofWorkflow {
    pub(crate) fn required_gate(self) -> &'static str {
        match self {
            Self::PlatformUpload => "artifact plus concrete platform ID",
            Self::LiveDeployment => "live app/job/pod identity plus success state",
            Self::LocalOnly => "command, exit state, and preserved artifact path if produced",
        }
    }
}

pub(crate) fn infer(text: &str) -> ProofWorkflow {
    let lower = text.to_ascii_lowercase();
    if lower.contains("youtube") || lower.contains("tiktok") || lower.contains("upload") {
        ProofWorkflow::PlatformUpload
    } else if lower.contains("argo") || lower.contains("kubectl") || lower.contains("cluster") {
        ProofWorkflow::LiveDeployment
    } else {
        ProofWorkflow::LocalOnly
    }
}
