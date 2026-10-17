//! PRD dependencies operations.
use super::*;
impl Prd {
    /// Check if all dependencies are met (all passed)
    pub(super) fn dependencies_met(&self, deps: &[String]) -> bool {
        deps.iter().all(|dep_id| {
            self.user_stories
                .iter()
                .find(|s| s.id == *dep_id)
                .map(|s| s.passes)
                .unwrap_or(true)
        })
    }
}
