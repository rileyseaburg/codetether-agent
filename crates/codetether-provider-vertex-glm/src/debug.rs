//! Redacted provider diagnostics.
use crate::*;
impl std::fmt::Debug for VertexGlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexGlmProvider")
            .field("project_id", &self.project_id)
            .field("base_url", &self.base_url)
            .field("client_email", &self.auth.client_email())
            .finish()
    }
}
