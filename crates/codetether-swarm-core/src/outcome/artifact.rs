use super::*;

/// An artifact produced by the swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let artifact = codetether_swarm_core::SwarmArtifact { artifact_type: "text".into(), name: "summary".into(), content: "done".into(), source_subagent: None, mime_type: Some("text/plain".into()) };
/// assert_eq!(artifact.name, "summary");
/// ```
pub struct SwarmArtifact {
    /// Artifact type
    pub artifact_type: String,

    /// Name/identifier
    pub name: String,

    /// Content or path
    pub content: String,

    /// Which sub-agent produced it
    pub source_subagent: Option<String>,

    /// MIME type
    pub mime_type: Option<String>,
}
